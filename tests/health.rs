mod fixtures;

#[tokio::test]
async fn test_health() {
    // Given
    fixtures::spawn_app();

    let client = reqwest::Client::new();

    // Assert
    let resp = client
        .get(format!("{}", fixtures::listen_addr()))
        .send()
        .await
        .expect("couldnt't send request!!!");

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK)
}
