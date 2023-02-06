pub mod config;
pub mod env;
pub mod handlers;
pub mod subscriber;

use crate::app::config::AppConfig;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use handlers::*;
use lazy_static::lazy_static;
use std::net::TcpListener;

lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::new().expect("config can't be loaded!!!");
}

pub fn run() -> Result<Server, std::io::Error> {
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

    // metrics
    let prometheus = actix_web_prom::PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    // state
    let web_data = web::Data::new(env::AppEnv::new(model));

    tracing::info!("staring sever...");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web_data.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(prometheus.clone())
            .service(health)
            .service(predict)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
