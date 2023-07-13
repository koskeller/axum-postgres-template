use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use uuid::Uuid;

use server::{run, setup_db, setup_tracing, Configuration};

static TRACING: Once = Once::new();

pub struct TestApp {
    pub addr: String,
    pub pool: PgPool,
    pub reqwest: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        // So tests can spawn multiple servers on OS assigned ports.
        std::env::set_var("PORT", "0");

        TRACING.call_once(setup_tracing);

        let cfg = Configuration::new().expect("Failed to read configuration");

        let url = create_temp_db(&cfg.db_dsn).await;
        let db = setup_db(&url, cfg.db_pool_max_size)
            .await
            .expect("Failed to configure db");

        let reqwest = reqwest::Client::new();

        let server = run(cfg, db.clone());
        let addr = format!("http://{}", server.local_addr());
        let _ = tokio::spawn(server);
        Self {
            pool: db,
            addr,
            reqwest,
        }
    }

    pub fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.addr, path)
    }
}

pub async fn create_temp_db(db_dsn: &str) -> String {
    let randon_db_name = Uuid::new_v4().to_string();
    let db_url = format!("{}/{}", &db_dsn, randon_db_name);
    let mut conn = PgConnection::connect(db_dsn)
        .await
        .expect("Failed to connect to Postgres");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, randon_db_name).as_str())
        .await
        .expect("Failed to create database");
    db_url
}
