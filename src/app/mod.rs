pub mod handlers;
pub mod state;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use handlers::*;
use state::AppState;

pub fn run(model: tch::CModule) -> Result<Server, std::io::Error> {
    // metrics
    let prometheus = actix_web_prom::PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    // state
    let web_data = web::Data::new(AppState::new(model));

    // server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web_data.clone())
            .wrap(prometheus.clone())
            .service(health)
            .service(predict)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
