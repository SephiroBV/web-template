use actix_web::{App, body::to_bytes, test};

use crate::utils::test_init::test_config;

#[actix_web::test]
async fn get_heartbeat() {
    // Arrange
    let app = test::init_service(App::new().configure(test_config)).await;
    let req = test::TestRequest::get().uri("/heartbeat").to_request();

    // Act
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let response_body = resp.into_body();
    assert_eq!("Up", to_bytes(response_body).await.unwrap());
}
