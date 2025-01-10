use actix_web::{get, web, HttpResponse};
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::community::Community;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/get/ownerships/{wallet}")]
pub async fn get_ownerships(data: web::Data<Mutex<Orchestrator>>, wallet: web::Path<String>) -> HttpResponse {
    let wallet = wallet.into_inner();
    let lock = data.lock().await;
    let ownerships = lock.get_ownerships(wallet).await;
    match ownerships {
        Ok(huy) => {
            HttpResponse::Ok().json(ApiResponse::<Vec<Community>> {
                msg: Option::from(String::from("Success")),
                data: Option::from(huy)
            })
        }
        Err(e) => {HttpResponse::InternalServerError().body(e.to_string())}
    }
}

