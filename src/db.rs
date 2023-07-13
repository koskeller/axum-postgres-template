use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

// We create a single connection pool for SQLx that's shared across the whole application.
// This saves us from opening a new connection for every API call, which is wasteful.
pub async fn setup_db(dsn: &str, pool_max_size: u32) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(pool_max_size)
        .acquire_timeout(Duration::from_secs(3))
        .connect(dsn)
        .await?;
    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup.
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
