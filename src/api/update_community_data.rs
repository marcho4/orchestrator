use actix_web::{ post, web, HttpResponse, Result as ActixResult};
use actix_web::error::ErrorInternalServerError;
use log::info;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::update_community_data::UpdateCommunityData;
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/{api_name}/update_community_data")]
pub async fn update_community_data(api_name: web::Path<String>, body: web::Json<UpdateCommunityData>,
                                   orchestrator: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let api_name = api_name.into_inner();
    let body = body.into_inner();
    let orchestrator = orchestrator.lock().await;
    let url = format!("http://community_service:8003/community/update/{}", api_name);
    let microservice_response = orchestrator.client.post(url).json(&body).send().await
        .map_err(|e| ErrorInternalServerError(e))?;

    if microservice_response.status().is_success() {
        let text = microservice_response.text().await
            .map_err(|e| ErrorInternalServerError(e))?;
        info!("{}", text);
        Ok(HttpResponse::Ok().json(ApiResponse::<String> {
            msg: Some("Successfully changed data".to_string()),
            data: None
        }))
    } else {
        Ok(HttpResponse::InternalServerError().json(ApiResponse::<String> {
            msg: Some("Error while updating community's data".to_string()),
            data: None
        }))
    }
}