mod fixtures;

use actix_web::{http::StatusCode, web, App};
use rl_proto::app;
use rl_proto::app::{env::AppEnv, handlers::predict};

#[actix_web::test]
async fn test_predict() {
    use actix_web::test;

    // Given
    fixtures::spawn_app();

    let client = reqwest::Client::new();

    // When
    let observation: serde_json::Value = serde_json::from_str(
        r#"{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}"#,
    ).expect("Couldn't parse observation json");

    // Assert
    let resp = client
        .post(format!("{}/predict", fixtures::listen_addr()))
        .json(&observation)
        .send()
        .await
        .expect("couldnt't send response!!!");

    assert_eq!(resp.status(), StatusCode::OK)
}
