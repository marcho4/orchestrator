use actix_web::{get, web, HttpResponse};
use crate::orchestrator::orchestrator::Orchestrator;

#[get("login/{wallet}")]
pub async fn login(data: web::Data<Orchestrator>, wallet: String) -> HttpResponse {
    data.process_login(wallet).await
}