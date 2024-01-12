use serde::Deserialize;
use std::{
    env::var,
    net::{Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

pub type Config = Arc<Configuration>;

#[derive(Deserialize)]
pub struct Configuration {
    /// The environment to run the application in.
    pub env: Environment,

    /// The address to listen on.
    pub listen_address: SocketAddr,
    /// The port to listen on.
    pub app_port: u16,

    /// The DSN for the database. Only postgres is currently supported.
    pub db_dsn: String,
    /// The maximum number of connections for the PostgreSQL pool.
    pub db_pool_max_size: u32,
}

#[derive(Deserialize, Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Configuration {
    /// Creates a new configuration from environment variables.
    pub fn new() -> Config {
        let app_port = var("PORT")
            .expect("Missing PORT environment variable")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

        let db_dsn = var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");

        let db_pool_max_size = var("DATABASE_POOL_MAX_SIZE")
            .expect("Missing DATABASE_POOL_MAX_SIZE environment variable")
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer.");

        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        let env = var("APP_ENVIRONMENT")
            .expect("Missing APP_ENVIRONMENT environment variable")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

        Arc::new(Configuration {
            env,
            listen_address,
            app_port,
            db_dsn,
            db_pool_max_size,
        })
    }

    /// Sets the database DSN.
    /// This method is used from the tests to override the database DSN.
    pub fn set_dsn(&mut self, db_dsn: String) {
        self.db_dsn = db_dsn
    }
}

impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "Invalid environment: {}. Please make sure it is either \"development\" or \"production\".",
                s
            )),
        }
    }
}
