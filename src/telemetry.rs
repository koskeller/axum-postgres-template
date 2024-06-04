use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// The `EnvFilter` type is used to filter log events based on the value of an environment variable.
/// In this case, we are using the `try_from_default_env` method to attempt to read the `RUST_LOG` environment variable,
/// which is used to set the log level for the application.
/// If the environment variable is not set, we default to the log level of `debug`.
/// The `RUST_LOG` environment variable is set in the Dockerfile and .env files.
pub fn setup_tracing() {
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());
    let formatting_layer = fmt::layer().json();
    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(formatting_layer)
        .init()
}

/// Returns a `TraceLayer` for HTTP requests and responses.
/// The `TraceLayer` is used to trace requests and responses in the application.
pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
