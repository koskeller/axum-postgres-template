use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

mod health_check;
pub(crate) use health_check::health_check_handler;

mod example;
pub(crate) use example::example_handler;

use crate::telemetry;

pub fn router(pool: PgPool) -> Router {
    let trace_layer = telemetry::trace_layer();
    let (req_headers_layer, resp_headers_layer) = telemetry::sensitive_headers_layers();

    Router::new()
        .route("/health_check", get(health_check_handler))
        .route("/example", post(example_handler))
        .layer(resp_headers_layer)
        .layer(trace_layer)
        .layer(req_headers_layer)
        .with_state(pool)
}
