use actix_web::web;

use web_template::init::app_config;

pub fn test_config(cfg: &mut web::ServiceConfig) {
    app_config::config(cfg);
}
