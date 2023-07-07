use axum::{http::StatusCode, response::IntoResponse};
use tracing::log::info;

pub async fn health_check_handler() -> impl IntoResponse {
    info!("health check request");
    StatusCode::OK
}
