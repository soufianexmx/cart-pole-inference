use super::env::AppEnv;
use crate::data::action::Action;
use crate::data::observation::Observation;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use tch::nn::Module;

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/predict")]
pub async fn predict(
    observation: web::Json<Observation>,
    env: web::Data<AppEnv>,
) -> Result<impl Responder> {
    let action = Action::from(env.model().forward(&observation.0.into()));

    Ok(web::Json(action))
}
