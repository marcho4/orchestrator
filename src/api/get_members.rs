use actix_web::{get, web, HttpResponse, Result as ActixResult};
use actix_web::error::ErrorInternalServerError;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::license::License;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/{api_name}/members")]
pub async fn fetch_all_community_members(api_name: web::Path<String>, data: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let api_name = api_name.into_inner();
    let url = format!("http://license_service:8001/license/{api_name}/all");
    let response = data.lock().await.client.get(url).send().await.map_err(
        |_| ErrorInternalServerError("Error with request to license service")
    )?;
    let json = response.json::<ApiResponse<Vec<License>>>().await.map_err(
        |_| ErrorInternalServerError("No members")
    )?;
    if json.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
            msg: Some("Success".to_string()),
            data: Some(json.data.unwrap())
        }))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
            msg: Some("No data was found".to_string()),
            data: Some(Vec::new())
        }))
    }
}