use axum::Router;

pub mod api_error;
pub mod cfg;
pub mod db;
pub mod middleware;
pub mod routes;
pub mod telemetry;

pub use cfg::*;
pub use db::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cfg: Config,
}

pub fn router(cfg: Config, db: Db) -> Router {
    let app_state = AppState { db, cfg };

    // Middleware that adds high level tracing to a Service.
    // Trace comes with good defaults but also supports customizing many aspects of the output:
    // https://docs.rs/tower-http/latest/tower_http/trace/index.html
    let trace_layer = telemetry::trace_layer();

    // Sets 'x-request-id' header with randomly generated uuid v7.
    let request_id_layer = middleware::request_id_layer();

    // Propagates 'x-request-id' header from the request to the response.
    let propagate_request_id_layer = middleware::propagate_request_id_layer();

    // Layer that applies the Cors middleware which adds headers for CORS.
    let cors_layer = middleware::cors_layer();

    // Layer that applies the Timeout middleware, which sets a timeout for requests.
    // The default value is 15 seconds.
    let timeout_layer = middleware::timeout_layer();

    // Any trailing slashes from request paths will be removed. For example, a request with `/foo/`
    // will be changed to `/foo` before reaching the internal service.
    let normalize_path_layer = middleware::normalize_path_layer();

    // Create the router with the routes.
    let router = routes::router();

    // Combine all the routes and apply the middleware layers.
    // The order of the layers is important. The first layer is the outermost layer.
    Router::new()
        .merge(router)
        .layer(normalize_path_layer)
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(propagate_request_id_layer)
        .layer(trace_layer)
        .layer(request_id_layer)
        .with_state(app_state)
}
