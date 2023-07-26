use server::{setup_db, setup_tracing, Configuration};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // Loads the .env file located in the environment's current directory or its parents in sequence.
    // .env used only for development, so we discard error in all other cases.
    dotenv::dotenv().ok();

    // Tries to load tracing config from environment (RUST_LOG) or uses "debug".
    setup_tracing();

    // Parse configuration from the environment.
    tracing::debug!("Initializing configuration");
    let cfg = Configuration::new();

    // Initialize db and run migrations.
    tracing::debug!("Initializing db pool");
    let db = setup_db(&cfg.db_dsn, cfg.db_pool_max_size).await;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup.
    tracing::debug!("Running migrations");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    // Spin up our server.
    tracing::info!("Starting server on {}...", cfg.listen_address);
    server::run(cfg, db).await
}
