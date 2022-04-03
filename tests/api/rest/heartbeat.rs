use actix_web::{App, body::to_bytes, test};

use crate::utils::test_init::test_config;

#[actix_web::test]
async fn get_heartbeat() {
    // Arrange
    let app = test::init_service(App::new().configure(test_config)).await;
    let request = test::TestRequest::get().uri("/heartbeat").to_request();

    // Act
    let response = test::call_service(&app, request).await;

    // Assert
    assert!(response.status().is_success());
    let body = response.into_body();
    assert_eq!("Up", to_bytes(body).await.unwrap());
}
