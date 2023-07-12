use axum::{routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::SocketAddr;

mod configuration;
pub use configuration::*;
mod telemetry;
pub use telemetry::*;
mod middleware;
pub use middleware::*;
mod db;
pub use db::*;
mod routes;

pub fn serve(addr: SocketAddr, pool: PgPool) -> Server<AddrIncoming, IntoMakeService<Router>> {
    axum::Server::bind(&addr).serve(routes::router(pool).into_make_service())
}
