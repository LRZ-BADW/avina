//! Types that define the configuration parameters and functions to retrieve them.
//!
//! The structure of the configuration is laid out in structs named `.*Settings`,
//! that implement [serde::Deserialize]. The [config] crate is used to parse this
//! either from a configuration file or environment variables.

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};

/// Wrapper combining all other settings.
#[derive(Clone, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub openstack: OpenStackSettings,
}

/// Settings for the application, the API backend, itself.
///
/// Contains things like the host and port the backend should
/// bind to, but also login data for other helper services, such
/// as avina-ldap.
#[derive(Clone, serde::Deserialize)]
pub struct ApplicationSettings {
    /// Port to listen on.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// Host address to bind to.
    pub host: String,
    /// Base URL of this API backend.
    pub base_url: String,
    /// Whether the admin user should be inserted by default.
    pub insert_admin: bool,
    /// API URL of the cloudusage service.
    pub cloud_usage_url: Option<String>,
    /// API URL of the avina-ldap service.
    pub avina_ldap_url: Option<String>,
    /// Token for accessing the avina-ldap service.
    pub avina_ldap_token: Option<String>,
    /// Whether defaults should be used, when a user/project is not found via avina-ldap.
    pub avina_ldap_default: Option<bool>,
}

/// Deserialize a string containing a secret.
///
/// The [SecretString] type wraps a [String] and prevents it from
/// being leaked via printing or logging.
fn deserialize_secret_string<'de, D>(
    deserializer: D,
) -> Result<SecretString, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SecretString::from(s))
}

/// Settings needed for accessing the MariaDB database.
#[derive(Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    /// Username of the database user.
    pub username: String,
    /// Password of the database user.
    #[serde(deserialize_with = "deserialize_secret_string")]
    pub password: SecretString,
    /// Port the database listens on.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// Host address the database listens on.
    pub host: String,
    /// Name of the database used for avina.
    pub database_name: String,
    /// Whether the backend should require an encrypted connection to the database.
    pub require_ssl: bool,
}

/// Settings needed for accessing the OpenStack services.
#[derive(Clone, serde::Deserialize)]
pub struct OpenStackSettings {
    /// Username of the OpenStack admin account.
    pub username: String,
    /// Password of the OpenStack admin account.
    pub password: String,
    /// Project name of the OpenStack admin account.
    pub project: String,
    /// Project ID of the of OpenStack admin account.
    pub project_id: String,
    /// Domain name of the OpenStack admin account.
    pub domain: String,
    /// Domain ID of the OpenStack admin account.
    pub domain_id: String,
    /// API URL of the Keystone service.
    pub keystone_endpoint: String,
    /// API URL of the Nova service.
    pub nova_endpoint: String,
}

impl DatabaseSettings {
    /// Connection options excluding the database name.
    ///
    /// This does not connect to a specific database. This is necessary,
    /// for example, when the database has to be created first.
    pub fn without_db(&self) -> MySqlConnectOptions {
        let ssl_mode = if self.require_ssl {
            MySqlSslMode::Required
        } else {
            MySqlSslMode::Preferred
        };
        MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.username)
            .password(self.password.expose_secret())
    }

    /// Connection options including  the database name.
    ///
    /// This connects to the specified database.
    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

/// Read the settings from the configuration file or environment variables.
///
/// The final settings are interpolated from the `base.yaml` and `local.yaml`
/// or `production.yaml`, depending on the environment set in the `APP_ENVIRONMENT`
/// environment variable, and environment variables with the prefix `APP`.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine current directory.");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

/// The potential environments the API backend may be executed in (local or production).
pub enum Environment {
    /// Local execution, i.e., on the machine of the developer.
    Local,
    /// Execution in the production environment.
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a supported environment. \
                Use either 'local' or 'production'."
            )),
        }
    }
}
