use std::net::SocketAddr;

use server::{configure_db, get_subscriber, init_subscriber, Settings};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let cfg = Settings::new().expect("Failed to read configuration");

    let subscriber = get_subscriber("info".into());
    init_subscriber(subscriber);

    let pool = configure_db(cfg.db_url)
        .await
        .expect("Failed to configure db");

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], cfg.app_port));
    server::serve(addr, pool).await
}
