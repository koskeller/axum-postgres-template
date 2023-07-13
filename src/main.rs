use server::{setup_db, setup_tracing, Configuration};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Initialize tracing.
    setup_tracing();

    // Parse our configuration from the environment.
    let cfg = Configuration::new().expect("Failed to read configuration");

    // TODO move migrations somewhere else?
    tracing::debug!("DB: Initializing pool");
    let db = setup_db(&cfg.db_dsn, cfg.db_pool_max_size)
        .await
        .expect("Failed to setup db");
    tracing::debug!("DB: Started");

    // Finally, we spin up our API.
    server::run(cfg, db).await
}
