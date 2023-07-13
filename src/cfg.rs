use std::{
    env::var,
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};

pub type Config = Arc<Configuration>;

#[derive(serde::Deserialize)]
pub struct Configuration {
    /// The address to listen on.
    pub listen_address: SocketAddr,
    // The port to listen on.
    pub app_port: u16,

    /// The DSN for the database. Only postgres is currently supported.
    // pub db_dsn: String,
    pub db_dsn: String,
    // The maximum number of connections for the PostgreSQL pool
    pub db_pool_max_size: u32,
}

impl Configuration {
    pub fn new() -> anyhow::Result<Config> {
        let app_port = var("PORT")?.parse::<u16>()?;
        let db_dsn = var("DATABASE_URL")?;
        let db_pool_max_size = var("DATABASE_POOL_MAX_SIZE")?.parse::<u32>()?;

        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        Ok(Arc::new(Configuration {
            listen_address,
            app_port,
            db_dsn,
            db_pool_max_size,
        }))
    }

    pub fn set_dsn(&mut self, db_dsn: String) {
        self.db_dsn = db_dsn
    }
}
