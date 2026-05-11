//! Top-level module of the avina-api application.
//!
//! It imports everything relevant from the also shipped library,
//! and just initializes and starts it in [main].

use avina_api::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

/// Main method and entry point of the avina-api application.
///
/// It just initializes the tracing, retrieves the configuration,
/// and then builds and runs the application.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber =
        get_subscriber("avina-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
