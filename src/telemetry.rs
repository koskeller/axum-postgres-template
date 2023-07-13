use hyper::header;
use std::sync::Arc;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{Level, Subscriber};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn get_subscriber() -> impl Subscriber + Send + Sync {
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());
    let formatting_layer = fmt::layer().json();

    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    subscriber.init()
}

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
