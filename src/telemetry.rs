use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// The `EnvFilter` type is used to filter log events based on the value of an environment variable.
// In this case, we are using the `try_from_default_env` method to attempt to read the `RUST_LOG` environment variable,
// which is used to set the log level for the application.
// If the environment variable is not set, we fall back to the default log level of `debug`.
pub fn setup_tracing() {
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());
    let formatting_layer = fmt::layer().json();
    tracing_subscriber::registry()
        .with(env_filter_layer)
        .with(formatting_layer)
        .init()
}
