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

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
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

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            openstack,
            configuration.application.cloud_usage_url,
        )
        .await?;

        Ok(Self { port, server })
    }

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

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);
#[derive(Debug)]
pub struct CloudUsageUrl(pub Option<String>);

async fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    base_url: String,
    openstack: OpenStack,
    cloud_usage_url: Option<String>,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let openstack = Data::new(openstack);
    let cloud_usage_url = Data::new(CloudUsageUrl(cloud_usage_url));
    let quota_cache = Data::new(Mutex::new(QuotaCache::new()));
    let server = HttpServer::new(move || {
        // TODO: this should be configurable
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
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

pub fn get_connection_pool(configuration: &DatabaseSettings) -> MySqlPool {
    MySqlPoolOptions::new().connect_lazy_with(configuration.with_db())
}
