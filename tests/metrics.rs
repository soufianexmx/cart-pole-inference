mod fixtures;

#[tokio::test]
async fn test_metrics() {
    // Given
    fixtures::spawn_app();

    let client = reqwest::Client::new();

    // Assert
    let resp = client
        .get(format!("{}/metrics", fixtures::listen_addr()))
        .send()
        .await
        .expect("couldnt't send request!!!");

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK)
}
