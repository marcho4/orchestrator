use actix_web::{get, web, HttpResponse, Result as ActixResult};
use actix_web::error::ErrorInternalServerError;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/{api_name}/check/{wallet}")]
pub async fn check_wallet(data: web::Data<Mutex<Orchestrator>>, path: web::Path<(String, String)>) -> ActixResult<HttpResponse> {
    let orchestrator = data.lock().await;
    let (api_name, wallet) = path.into_inner();
    let url = format!("http://community_service:8003/community/{api_name}/check/{wallet}");
    let req = orchestrator.client.get(url).send().await.map_err(|e| {
        ErrorInternalServerError(format!("Error getting community data: {}", e))
    })?.json::<ApiResponse<bool>>().await.map_err(|e| {
        ErrorInternalServerError(format!("Error getting community data: {}", e))
    })?;
    if req.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<bool> {
            msg: Option::from(String::from("Success")),
            data: req.data
        }))
    } else {
        Ok(HttpResponse::BadRequest().json(ApiResponse::<bool> {
            msg: Option::from(format!("No such wallet")),
            data: None
        }))
    }
}