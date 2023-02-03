use actix_web::{http::StatusCode, test, web, App};
use rl_proto::app::{handlers::predict, state::AppState};
use rl_proto::data::observation::Observation;

#[actix_web::test]
async fn test_predict() {
    // Given
    let mut model = tch::CModule::load("CartPole-v1.pt").expect("Couldn't load module");
    model.set_eval();

    let web_data = web::Data::new(AppState::new(model));
    let app = test::init_service(App::new().app_data(web_data.clone()).service(predict)).await;

    // When
    let observation = Observation::new(2.0, 20.0, 0.1, 10.0);

    let req = test::TestRequest::post()
        .uri("/predict")
        .set_json(observation)
        .to_request();

    // Assert
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK)
}
