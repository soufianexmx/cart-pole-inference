use actix_web::{http::StatusCode, App};
use rl_proto::app::handlers::health;

#[actix_web::test]
async fn test_health() {
    use actix_web::test;

    // Given
    let app = test::init_service(App::new().service(health)).await;

    // When
    let req = test::TestRequest::get().uri("/").to_request();

    // Assert
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK)
}
