use axum::Router;
use std::sync::Arc;

mod cfg;
pub use cfg::*;
mod telemetry;
pub use telemetry::*;
mod middleware;
pub use middleware::*;
mod db;
pub use db::*;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cfg: Arc<Configuration>,
}

pub fn router(cfg: Config, db: Db) -> Router {
    let app_state = AppState { db, cfg };

    // Middleware that adds high level tracing to a Service.
    // Trace comes with good defaults but also supports customizing many aspects of the output:
    // https://docs.rs/tower-http/latest/tower_http/trace/index.html
    let trace_layer = telemetry::trace_layer();

    // Hiding sensitive headers is a good security practice as it prevents sensitive information
    // such as authorization tokens and cookies from being leaked to unauthorized parties.
    let (req_headers_layer, resp_headers_layer) = telemetry::sensitive_headers_layers();

    // Sets 'x-request-id' header with randomly generated uuid v4.
    let request_id_layer = middleware::request_id_layer();

    // Propagates 'x-request-id' header from the request to the response.
    let propagate_request_id_layer = middleware::propagate_request_id_layer();

    // Layer that applies the Cors middleware which adds headers for CORS.
    let cors_layer = middleware::cors_layer();

    // Layer that applies the Timeout middleware which apply a timeout to requests.
    // Default value is 15 seconds.
    let timeout_layer = middleware::timeout_layer();

    Router::new()
        .merge(routes::router())
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(resp_headers_layer)
        .layer(propagate_request_id_layer)
        .layer(trace_layer)
        .layer(req_headers_layer)
        .layer(request_id_layer)
        .with_state(app_state)
}
