use actix_web::{get, Responder};

#[get("/heartbeat")]
pub async fn get_heartbeat() -> impl Responder {
    "Up"
}
