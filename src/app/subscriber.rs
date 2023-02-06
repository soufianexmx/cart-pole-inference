use crate::app::CONFIG;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

fn get_subscriber() -> impl Subscriber {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(CONFIG.log_level()));

    let formatting_layer = BunyanFormattingLayer::new("rl-proto".into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber() {
    tracing::subscriber::set_global_default(get_subscriber()).expect("failed to set subscriber!!!");
}
