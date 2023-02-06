mod fixtures;

#[tokio::test]
async fn test_predict() {
    // Given
    let env = fixtures::spawn_app();

    let client = reqwest::Client::new();

    // When
    let observation: serde_json::Value = serde_json::from_str(
        r#"{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}"#,
    ).expect("Couldn't parse observation json");

    // Assert
    let resp = client
        .post(format!("{}/predict", env.listen_addr()))
        .json(&observation)
        .send()
        .await
        .expect("couldnt't send request!!!");

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK)
}
