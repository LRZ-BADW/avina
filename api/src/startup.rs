//! Application definition and start-up.
//!
//! [Application] is the main struct for `avina-api` execution.

use std::{net::TcpListener, sync::Mutex};

use actix_cors::Cors;
use actix_web::{
    App, HttpServer, dev::Server, middleware::from_fn, web, web::Data,
};
use anyhow::Context;
use avina_wire::user::UserClass;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tracing_actix_web::TracingLogger;

use crate::{
    authentication::{extract_user_and_project, require_valid_token},
    configuration::{DatabaseSettings, Settings},
    error::{MinimalApiError, not_found},
    openstack::OpenStack,
    routes::{
        accounting_scope, budgeting_scope, health_check, hello_scope,
        pricing_scope,
        quota::flavor_quota::check::QuotaCache,
        quota_scope, resources_scope,
        user::{
            project::create::{NewProject, insert_project_into_db},
            user::create::{NewUser, insert_user_into_db},
        },
        user_scope,
    },
};

/// Instance of the `avina-api` application.
///
/// This is the central struct for initializing all auxiliary objects, and building and
/// starting the server function. This abstraction is used both by the `avina-api` binary and the
/// `avina-test` crate.
pub struct Application {
    /// Port the server is supposed to listen on.
    port: u16,
    /// The [actix_web] server, serving the API backend.
    server: Server,
}

impl Application {
    /// Build the application from the given settings.
    ///
    /// This initializes auxiliary objects, e.g., database connection, OpenStack and LDAP helper
    /// services, and then starts and runs the web server. Depending on the configuration, it also
    /// inserts a default admin user into the database, which is helpful for local test deployments.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        if configuration.application.insert_admin {
            Self::insert_admin_user(&connection_pool, &configuration).await?;
        }

        let openstack = OpenStack::new(configuration.openstack).await?;
        let avina_ldap_config = AvinaLdapConfig::new(
            configuration.application.avina_ldap_url,
            configuration.application.avina_ldap_token,
            configuration.application.avina_ldap_default,
        );

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            openstack,
            configuration.application.cloud_usage_url,
            avina_ldap_config,
        )
        .await?;

        Ok(Self { port, server })
    }

    /// Inserts the default admin user of `avina-api` in the database.
    ///
    /// It uses the configured OpenStack admin project as default admin user.
    /// In case the corresponding project or user already exists, it simply
    /// skips insertion with a log message.
    async fn insert_admin_user(
        connection_pool: &MySqlPool,
        configuration: &Settings,
    ) -> Result<(), anyhow::Error> {
        let mut transaction = connection_pool
            .begin()
            .await
            .context("Failed to begin transaction")?;
        let project = NewProject {
            name: configuration.openstack.domain.clone(),
            openstack_id: configuration.openstack.domain_id.clone(),
            user_class: UserClass::UC1,
        };
        let project_id =
            match insert_project_into_db(&mut transaction, &project).await {
                Ok(project_id) => project_id,
                Err(MinimalApiError::ValidationError(_)) => {
                    tracing::info!("Admin project already exists, skipping.");
                    return Ok(());
                }
                Err(MinimalApiError::UnexpectedError(e)) => {
                    return Err(e);
                }
            };
        let user = NewUser {
            name: configuration.openstack.project.clone(),
            openstack_id: configuration.openstack.project_id.clone(),
            project_id: project_id as u32,
            role: 1,
            is_staff: true,
            is_active: true,
        };
        let _user_id = match insert_user_into_db(&mut transaction, &user).await
        {
            Ok(user_id) => user_id,
            Err(MinimalApiError::ValidationError(_)) => {
                tracing::info!("Admin user already exists, skipping.");
                return Ok(());
            }
            Err(MinimalApiError::UnexpectedError(e)) => {
                return Err(e);
            }
        };
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(())
    }

    /// Get the port the application is bound to.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Run the server until it is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Wrapper type for the application base URL.
///
/// As this is handed to endpoints as [Data], it needs to have a distinguishable type.
pub struct ApplicationBaseUrl(pub String);

/// Wrapper type for the cloudusage URL.
///
/// As this is handed to endpoints as [Data], it needs to have a distinguishable type.
#[derive(Debug)]
pub struct CloudUsageUrl(pub Option<String>);

/// Configuration abstraction for avina-ldap access.
#[derive(Debug)]
pub enum AvinaLdapConfig {
    /// Access to avina-ldap is enabled, and the contained API URL and token (given by the first and
    /// second [String]) are used. Using defaults in absence of other data is specified in the
    /// [bool].
    Enabled(String, String, bool),
    /// Access to avina-ldap is disabled, and using defaults instead is specified in the contained
    /// [bool].
    Disabled(bool),
}

impl AvinaLdapConfig {
    /// Build a new instance from the given or not given parameters.
    ///
    /// All parameters are optional, `default` defaults to [true], in absence of either the `url` or
    /// `token`, access is disabled.
    fn new(
        url: Option<String>,
        token: Option<String>,
        default: Option<bool>,
    ) -> Self {
        let default = default.unwrap_or(true);
        match (url, token) {
            (Some(url), Some(token)) => Self::Enabled(url, token, default),
            _ => Self::Disabled(default),
        }
    }
}

/// Setup and run the [HttpServer] with the supplied state objects and all imported endpoints.
///
/// This function packs all relevant data and functionality together in the resulting server. This
/// includes wrapping all state objects in [Data], setting up CORS (Cross-Origin Resource Sharing),
/// handing in the logger, and registering all the implemented API endpoints. It returns the
/// asynchronously running server.
async fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    base_url: String,
    openstack: OpenStack,
    cloud_usage_url: Option<String>,
    avina_ldap_data: AvinaLdapConfig,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let openstack = Data::new(openstack);
    let cloud_usage_url = Data::new(CloudUsageUrl(cloud_usage_url));
    let quota_cache = Data::new(Mutex::new(QuotaCache::new()));
    let avina_ldap_data = Data::new(avina_ldap_data);
    let server = HttpServer::new(move || {
        // TODO: this should be configurable
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://tcc.cloud.mwn.de:1339")
            .allowed_origin("https://cc.lrz.de:1339")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();
        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(openstack.clone())
            .app_data(cloud_usage_url.clone())
            .app_data(quota_cache.clone())
            .app_data(avina_ldap_data.clone())
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .wrap(from_fn(extract_user_and_project))
                    .wrap(from_fn(require_valid_token))
                    .route("/secured_health_check", web::get().to(health_check))
                    .service(hello_scope())
                    .service(user_scope())
                    .service(accounting_scope())
                    .service(resources_scope())
                    .service(pricing_scope())
                    .service(budgeting_scope())
                    .service(quota_scope()),
            )
            .default_service(web::route().to(not_found))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

/// Setup a connection pool with the configured database.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> MySqlPool {
    MySqlPoolOptions::new().connect_lazy_with(configuration.with_db())
}
