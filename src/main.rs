use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use tracing_actix_web::TracingLogger;

use web_template::init::{app_config, telemetry_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    telemetry_config::init("web_template");

    let client = ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware)
        .build();

    let app_factory = move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(app_config::config)
            .app_data(client.clone())
    };
    let server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        HttpServer::new(app_factory).listen(listener)?
    } else {
        //Google Cloud Run Port Injection
        let address = match std::env::var("PORT") {
            Ok(port) => format!("0.0.0.0:{}", port),
            Err(_e) => "127.0.0.1:8000".into(),
        };
        HttpServer::new(app_factory).bind(address)?
    };
    tracing::info!("Starting app, listening on {:?}", server.addrs());

    server.run().await
}
