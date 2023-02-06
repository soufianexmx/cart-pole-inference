mod config;
pub mod env;
mod handlers;
pub mod subscriber;

use crate::app::config::AppConfig;
use crate::app::env::AppEnv;
use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use handlers::*;
use lazy_static::lazy_static;
use std::net::TcpListener;

lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::new().expect("config can't be loaded!!!");
}

pub fn run() -> Result<(Server, Data<AppEnv>), std::io::Error> {
    tracing::info!(
        "setting up listener {}:{}...",
        CONFIG.address(),
        CONFIG.port()
    );
    let listener = TcpListener::bind((CONFIG.address(), CONFIG.port()))
        .expect("failed to bind random port!!!");

    let port = listener.local_addr().unwrap().port();

    tracing::info!("loading RL model {}...", CONFIG.model_path());
    let mut model = tch::CModule::load(CONFIG.model_path()).expect("Couldn't load module!!!");

    tracing::info!("set model to evaluation mode for inference...");
    model.set_eval();

    //env
    let env = Data::new(AppEnv::new(model, CONFIG.address().to_string(), port));
    let cloned_env = env.clone();

    // metrics
    let prometheus = actix_web_prom::PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    tracing::info!("staring sever...");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(env.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(prometheus.clone())
            .service(health)
            .service(predict)
    })
    .listen(listener)?
    .run();

    Ok((server, cloned_env))
}
