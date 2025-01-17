use std::time::Duration;
use crate::orchestrator::orchestrator::Orchestrator;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use tokio::sync::Mutex;
use actix_cors::Cors;
use crate::api::middleware::AuthMiddleware;

mod orchestrator;
mod models;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init Orchestrator
    let orchestrator = Mutex::new(Orchestrator::new());

    // Creating State with orchestrator
    let state = web::Data::new(orchestrator);
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start API
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(AuthMiddleware::new())
            .wrap(Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
            )
            .configure(api::routes::api_config)
    })
        .keep_alive(Duration::from_secs(75))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
