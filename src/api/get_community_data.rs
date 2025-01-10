use actix_web::{get, web, HttpResponse, Result as ActixResult};
use actix_web::error::ErrorBadGateway;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, PubCommunityInfo};
use crate::models::community::Community;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/community/{api_name}")]
pub async fn fetch_community_info(data: web::Data<Mutex<Orchestrator>>, api_name: web::Path<String>) -> ActixResult<HttpResponse> {
    let orchestrator = data.lock().await;
    let api_name = api_name.into_inner();

    let req = orchestrator.client
        .get(format!("http://community_service:8003/community/{}", api_name))
        .send().await.map_err(|e| {ErrorBadGateway("Error getting community data")})?;

    let data = match req.json::<ApiResponse<Community>>().await
        .map_err(|e| {ErrorBadGateway("Error getting community data")})?
        .data {
        Some(data) => data,
        None => return Ok(HttpResponse::BadRequest().json(ApiResponse::<Community> {
            msg: Option::from(String::from("There is no community with this name")),
            data: None
        }))
    };



    Ok(HttpResponse::Ok().json(ApiResponse::<PubCommunityInfo> {
        msg: Option::from(String::from("Success")),
        data: Option::from(
            PubCommunityInfo {
                name: data.name,
                api_name: data.api_name,
                description: data.description,
                logo: data.logo,
                social: data.social,
                price: data.price,
                renewal_period: data.renewal_period,
                collect_wallet: data.collect_wallet,
                need_wl: data.need_wl,
                plan: data.plan,
            }
        )
    }))
}

