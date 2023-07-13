use std::net::{Ipv6Addr, SocketAddr};

use server::{configure_db, get_subscriber, init_subscriber, Config};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let cfg = Config::new().expect("Failed to read configuration");

    // Initialize tracing.
    let subscriber = get_subscriber();
    init_subscriber(subscriber);

    let pool = configure_db(cfg.db_url)
        .await
        .expect("Failed to configure db");

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, cfg.app_port));

    // Finally, we spin up our API.
    server::serve(addr, pool).await
}
