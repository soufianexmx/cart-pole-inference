use lazy_static::lazy_static;
use rl_proto::app;
use rl_proto::app::config::AppConfig;
use std::net::TcpListener;

lazy_static! {
    static ref CONFIG: AppConfig = AppConfig::new().expect("config can't be loaded!!!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(CONFIG.log_level()))
        .init();

    let listener = TcpListener::bind((CONFIG.base_url(), CONFIG.port()))
        .expect("Failed to bind random port!!!");

    let mut model = tch::CModule::load(CONFIG.model_path()).expect("Couldn't load module!!!");
    model.set_eval();

    app::run(listener, model)?.await
}
