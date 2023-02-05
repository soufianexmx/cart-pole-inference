pub mod config;
pub mod env;
pub mod handlers;
pub mod subscriber;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use handlers::*;
use std::net::TcpListener;

pub fn run(listener: TcpListener, model: tch::CModule) -> Result<Server, std::io::Error> {
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
