use tracing_subscriber::{fmt, EnvFilter};
use std::env;

pub fn init_logging() {
    //Use RUST_LOG env var to set the log level
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    //Check if we're in production via APP_ENV
    let is_production = env::var("APP_ENV")
        .map(|v| v.to_lowercase() == "production")
        .unwrap_or(false);

    let subscriber: Box<dyn tracing::Subscriber + Send + Sync> = if is_production {
        Box::new(
            fmt::Subscriber::builder()
                .with_env_filter(filter)
                .json()
                .with_current_span(true)
                .with_span_events(fmt::format::FmtSpan::CLOSE)
                .flatten_event(true)
                .finish(),
        )
    } else {
        Box::new(
            fmt::Subscriber::builder()
                .with_env_filter(filter)
                .pretty()
                .with_target(true)
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_level(true)
                .finish(),
        )
    };

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default subscriber");
}

