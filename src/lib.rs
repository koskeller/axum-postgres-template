use axum::{routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::{net::SocketAddr, time::Duration};
use tower_http::timeout::TimeoutLayer;

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
    let trace_layer = telemetry::trace_layer();
    let (req_headers_layer, resp_headers_layer) = telemetry::sensitive_headers_layers();

    let request_id_layer = middleware::requiest_id_layer();
    let propagate_request_id_layer = middleware::propagate_requiest_id_layer();

    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let app = Router::new()
        .merge(routes::router())
        .layer(timeout_layer)
        .layer(resp_headers_layer)
        .layer(propagate_request_id_layer)
        .layer(trace_layer)
        .layer(req_headers_layer)
        .layer(request_id_layer)
        .with_state(pool);

    axum::Server::bind(&addr).serve(app.into_make_service())
}
