use actix_web::web;

use crate::routes::heartbeat::get_heartbeat;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_heartbeat);
}
