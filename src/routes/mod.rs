use axum::{
    routing::{get, post},
    Router,
};

mod health_check;
pub(crate) use health_check::health_check_handler;

mod example;
pub(crate) use example::example_handler;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check_handler))
        .route("/example", post(example_handler))
}
