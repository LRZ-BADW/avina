//! Initialization and helper functions for tracing.
//!
//! avina-api uses the [tracing] crate for structured logging.

use tokio::task::JoinHandle;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    EnvFilter, Registry, fmt::MakeWriter, layer::SubscriberExt,
};

/// Setup a tracing subscriber with given name, verbosity, and output.
///
/// # Arguments
///
///   - `name` - Name of the application.
///   - `env_filter` - Verbosity to filter messages. This is overwritten
///     by the `RUST_LOG` environment variable.
///   - `sink` - Place to write messages to.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Initialize the given tracing subscriber and set it a global default.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger.");
    set_global_default(subscriber).expect("Failed to set subscriber.");
}

/// Spawn a blocking (non-async) task in the context of the current tracing span.
///
/// This is necessary to ensure properties from current span are properly
/// inherited by the task.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
