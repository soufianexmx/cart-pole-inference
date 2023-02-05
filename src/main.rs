use lazy_static::lazy_static;
use rl_proto::app;
use rl_proto::app::config::AppConfig;
use std::net::TcpListener;

lazy_static! {
    static ref CONFIG: AppConfig = AppConfig::new().expect("config can't be loaded!!!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::subscriber::init_subscriber(CONFIG.log_level());

    tracing::info!(
        "setting up listener {}:{}...",
        CONFIG.base_url(),
        CONFIG.port()
    );
    let listener = TcpListener::bind((CONFIG.base_url(), CONFIG.port()))
        .expect("failed to bind random port!!!");

    tracing::info!("loading RL model {}...", CONFIG.model_path());
    let mut model = tch::CModule::load(CONFIG.model_path()).expect("Couldn't load module!!!");

    tracing::info!("set model to evaluation mode for inference...");
    model.set_eval();

    app::run(listener, model)?.await
}
