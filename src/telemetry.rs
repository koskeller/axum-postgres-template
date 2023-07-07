use tracing::Subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn get_subscriber(env_filer: String) -> impl Subscriber + Send + Sync {
    let env_filter_layer =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| env_filer.into());
    let formatting_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    subscriber.init()
}
