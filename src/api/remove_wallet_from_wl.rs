use actix_web::{web, HttpResponse, delete, Result as ActixResult};
use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use log::error;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, JwtClaims};
use crate::orchestrator::orchestrator::Orchestrator;

#[delete("/{api_name}/remove_from_wl/{wallet}")]
pub async fn remove_wallet_from_wl(api_name: web::Path<(String, String)>, orch: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let (api_name, wallet_name) = api_name.into_inner();
    let orchestrator = orch.lock().await;
    let url = format!("http://community_service:8003/community/ban/{api_name}");
    let resp = orchestrator.client.post(url).json(&JwtClaims {wallet: wallet_name}).send()
        .await.map_err(|e| ErrorInternalServerError(e))?;
    if resp.status().is_success() {
        Ok(HttpResponse::Ok().json(ApiResponse::<String> {msg: Some("Success".to_string()), data: None}))
    } else {
        error!("Error removing wallet from WALLET: {:?}", resp.status());
        Ok(HttpResponse::BadRequest().json(ApiResponse::<String> {msg: Some("Something went wrong".to_string()), data: None}))
    }
}
