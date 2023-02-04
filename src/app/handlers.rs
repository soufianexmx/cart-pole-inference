use super::env::AppEnv;
use crate::action::Action;
use crate::observation::Observation;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use tch::nn::Module;

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/predict")]
pub async fn predict(
    observation: web::Json<Observation>,
    state: web::Data<AppEnv>,
) -> Result<impl Responder> {
    let action = Action::from(state.model().forward(&observation.0.into()));

    Ok(web::Json(action))
}
