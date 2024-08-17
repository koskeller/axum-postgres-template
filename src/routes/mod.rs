use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub mod health_check;
pub mod home;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route("/", get(home::page))
        .nest_service("/public", ServeDir::new("public"))
}
