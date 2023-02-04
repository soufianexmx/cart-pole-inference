use actix_web::{http::StatusCode, web, App};
use rl_proto::app::{env::AppEnv, handlers::predict};

#[actix_web::test]
async fn test_predict() {
    use actix_web::test;

    // Given
    let mut model = tch::CModule::load("CartPole-v1.pt").expect("Couldn't load module");
    model.set_eval();

    let web_data = web::Data::new(AppEnv::new(model));
    let app = test::init_service(App::new().app_data(web_data.clone()).service(predict)).await;

    // When
    let observation: serde_json::Value = serde_json::from_str(
        r#"{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}"#,
    ).expect("Couldn't parse observation json");

    let req = test::TestRequest::post()
        .uri("/predict")
        .set_json(observation)
        .to_request();

    // Assert
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK)
}
