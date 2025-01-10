use actix_web::{get, web, HttpResponse, Result as ActixResult};
use actix_web::error::ErrorInternalServerError;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/{api_name}/wallets")]
pub async fn fetch_all_allowed_wallets(api_name: web::Path<String>, data: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let url = format!("http://community_service:8003/community/allowed_wallets/{api_name}");
    let response = data.lock().await.client.get(url).send().await.map_err(
        |_| ErrorInternalServerError("Error with request to community service")
    )?;
    let json = response.json::<ApiResponse<Vec<String>>>().await.map_err(
        |_| ErrorInternalServerError("No licenses")
    )?;
    if json.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<String>> {
            msg: Some("Success".to_string()),
            data: Some(json.data.unwrap())
        }))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<String>> {
            msg: Some("No data was found".to_string()),
            data: Some(Vec::new())
        }))
    }
}
