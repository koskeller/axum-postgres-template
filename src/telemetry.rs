use hyper::header;
use std::sync::Arc;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// The `EnvFilter` type is used to filter log events based on the value of an environment variable.
// In this case, we are using the `try_from_default_env` method to attempt to read the `RUST_LOG` environment variable,
// which is used to set the log level for the application.
// If the environment variable is not set, we fall back to the default log level of `debug`.
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
/// It includes headers and sets the log level to `INFO`.
pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_response(
            DefaultOnResponse::new()
                .include_headers(true)
                .level(Level::INFO),
        )
}
// Hiding sensitive headers is a good security practice as it prevents sensitive information
// such as authorization tokens and cookies from being leaked to unauthorized parties.
pub fn sensitive_headers_layers() -> (
    SetSensitiveRequestHeadersLayer,
    SetSensitiveResponseHeadersLayer,
) {
    let headers: Arc<[_]> = Arc::new([
        header::AUTHORIZATION,
        header::PROXY_AUTHORIZATION,
        header::COOKIE,
        header::SET_COOKIE,
    ]);

    let req = SetSensitiveRequestHeadersLayer::from_shared(Arc::clone(&headers));
    let resp = SetSensitiveResponseHeadersLayer::from_shared(headers);
    (req, resp)
}
