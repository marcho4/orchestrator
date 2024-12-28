use crate::orchestrator::orchestrator::Orchestrator;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use crate::api::login;

mod orchestrator;
mod models;
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init Orchestrator
    let orchestrator = Orchestrator::new();

    // Creating State with orchestrator
    let state = web::Data::new(orchestrator);
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start API
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())

            .service(
                web::scope("/api")
                    .service(login)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
