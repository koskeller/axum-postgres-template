use ::anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    pub async fn new(dsn: &str, pool_max_size: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(pool_max_size)
            .connect(dsn)
            .await?;
        Ok(Db { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        // This embeds database migrations in the application binary so we can ensure the database
        // is migrated correctly on startup.
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
