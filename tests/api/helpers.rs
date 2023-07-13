use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::{net::SocketAddr, sync::Once};
use uuid::Uuid;

use server::{configure_db, get_subscriber, init_subscriber, serve, Config};

static TRACING: Once = Once::new();

pub struct TestApp {
    pub pool: PgPool,
    pub addr: String,
    pub reqwest: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        let cfg = Config::new().expect("Failed to read configuration");

        TRACING.call_once(|| {
            let subscriber = get_subscriber();
            init_subscriber(subscriber);
        });

        let url = create_db(&cfg).await;
        let pool = configure_db(url).await.expect("Failed to configure db");

        let reqwest = reqwest::Client::new();

        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = serve(addr, pool.clone());
        let addr = format!("http://{}", server.local_addr());
        let _ = tokio::spawn(server);
        Self {
            pool,
            addr,
            reqwest,
        }
    }

    pub fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.addr, path)
    }
}

pub async fn create_db(cfg: &Config) -> String {
    let randon_db_name = Uuid::new_v4().to_string();
    let url = cfg.db_url_without_db();
    let db_url = format!("{}{}", url, randon_db_name);
    let mut conn = PgConnection::connect(&url)
        .await
        .expect("Failed to connect to Postgres");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, randon_db_name).as_str())
        .await
        .expect("Failed to create database");
    db_url
}
