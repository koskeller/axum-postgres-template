use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::SocketAddr;

mod configuration;
pub use configuration::*;

mod telemetry;
pub use telemetry::*;

mod db;
pub use db::*;

mod routes;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(routes::health_check_handler))
        .route("/example", post(routes::example_handler))
        .with_state(pool)
}

pub fn serve(addr: SocketAddr, pool: PgPool) -> Server<AddrIncoming, IntoMakeService<Router>> {
    axum::Server::bind(&addr).serve(app(pool).into_make_service())
}
