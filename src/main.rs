use crate::orchestrator::orchestrator::Orchestrator;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use tokio::sync::Mutex;
use crate::api::{login, request_nonce};
use actix_cors::Cors;
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
            .wrap(Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method())
            .service(
                web::scope("/api")
                    .service(login)
                    .service(request_nonce)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
