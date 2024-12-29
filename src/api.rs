use actix_web::{get, post, web, HttpResponse};
use tokio::sync::Mutex;
use crate::models::api_models::LoginData;
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/login")]
pub async fn login(data: web::Data<Mutex<Orchestrator>>, req_data: web::Json<LoginData>) -> HttpResponse {
    let login_data = req_data.into_inner();
    data.lock().await.process_login(login_data.wallet, login_data.signature).await
}

#[get("/auth/request_nonce/{wallet}")]
pub async fn request_nonce(data: web::Data<Mutex<Orchestrator>>, wallet: web::Path<String>) -> HttpResponse {
    let wallet = wallet.into_inner();
    let mut lock = data.lock().await;
    lock.request_nonce(wallet).await
}