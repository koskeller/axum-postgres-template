use axum::Router;
use std::sync::Arc;

mod cfg;
mod db;
pub mod middleware;
pub mod routes;
pub mod telemetry;

pub use cfg::*;
pub use db::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cfg: Arc<Configuration>,
}

pub fn router(cfg: Config, db: Db) -> Router {
    let app_state = AppState { db, cfg };

    // Sets 'x-request-id' header with randomly generated uuid v7.
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
        .layer(propagate_request_id_layer)
        .layer(request_id_layer)
        .with_state(app_state)
}
