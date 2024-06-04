use ::anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    // We create a single connection pool for SQLx that is shared across the entire application.
    // This prevents the need to open a new connection for every API call, which would be wasteful.
    pub async fn new(dsn: &str, pool_max_size: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(pool_max_size)
            .connect(dsn)
            .await?;
        Ok(Db { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        // This integrates database migrations into the application binary to ensure the database
        // is properly migrated during startup.
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
