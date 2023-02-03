pub mod handlers;
pub mod state;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use handlers::*;
use state::AppState;

pub fn run(model: tch::CModule) -> Result<Server, std::io::Error> {
    let web_data = web::Data::new(AppState::new(model));

    let server = HttpServer::new(move || App::new().app_data(web_data.clone()).service(predict))
        .bind(("127.0.0.1", 8080))?
        .run();

    Ok(server)
}
