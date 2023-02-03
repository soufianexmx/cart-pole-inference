use super::AppState;
use crate::action::Action;
use crate::observation::Observation;
use actix_web::{post, web, Responder, Result};
use tch::nn::Module;

#[post("/predict")]
pub async fn predict(
    observation: web::Json<Observation>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let action = Action::from(state.model().forward(&observation.0.into()));

    Ok(web::Json(action))
}
