use server::{telemetry, Configuration, Db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Loads the .env file located in the environment's current directory or its parents in sequence.
    // .env used only for development, so we discard error in all other cases.
    dotenvy::dotenv().ok();

    // Tries to load tracing config from environment (RUST_LOG) or uses "debug".
    telemetry::setup_tracing();

    // Parse configuration from the environment.
    // This will exit with a help message if something is wrong.
    tracing::debug!("Initializing configuration");
    let cfg = Configuration::new();

    // Initialize db pool.
    tracing::debug!("Initializing db pool");
    let db = Db::new(&cfg.db_dsn, cfg.db_pool_max_size)
        .await
        .expect("Failed to initialize db");

    tracing::debug!("Running migrations");
    db.migrate().await.expect("Failed to run migrations");

    // Spin up our server.
    tracing::info!("Starting server on {}", cfg.listen_address);
    let listener = TcpListener::bind(&cfg.listen_address)
        .await
        .expect("Failed to bind address");
    let router = server::router(cfg, db);
    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}
