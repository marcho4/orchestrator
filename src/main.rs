use crate::orchestrator::orchestrator::Orchestrator;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use tokio::sync::Mutex;
use actix_cors::Cors;
use crate::api::api::{check_wallet, create_community, fetch_all_allowed_wallets, fetch_all_community_members, fetch_community_info, get_memberships, get_ownerships};
use crate::api::auth_api::{get_session, login, logout, request_nonce};

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
                    .allowed_origin("http://localhost:3000")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
            )
            .service(
                web::scope("/api")
                    .service(login)
                    .service(request_nonce)
                    .service(get_session)
                    .service(get_ownerships)
                    .service(get_memberships)
                    .service(logout)
                    .service(create_community)
                    .service(fetch_community_info)
                    .service(check_wallet)
                    .service(fetch_all_community_members)
                    .service(fetch_all_allowed_wallets)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
