mod fixtures;
use actix_web::{http::StatusCode, App};
use rl_proto::app::handlers::health;

#[actix_web::test]
async fn test_health() {
    use actix_web::test;

    // Given
    fixtures::spawn_app();

    let client = reqwest::Client::new();

    // Assert
    let resp = client
        .get(format!("{}", fixtures::listen_addr()))
        .send()
        .await
        .expect("couldnt't send response!!!");

    assert_eq!(resp.status(), StatusCode::OK)
}
