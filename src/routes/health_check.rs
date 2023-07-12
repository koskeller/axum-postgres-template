use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check_handler() -> impl IntoResponse {
    StatusCode::OK
}
