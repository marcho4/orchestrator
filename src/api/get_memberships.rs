use actix_web::{get, web, HttpResponse};
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::license::License;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/get/memberships/{wallet}")]
pub async fn get_memberships(data: web::Data<Mutex<Orchestrator>>, wallet: web::Path<String>) -> HttpResponse {
    let wallet = wallet.into_inner();
    let lock = data.lock().await;
    let memberships = lock.get_memberships(wallet).await;
    match memberships {
        Ok(huy) => {
            HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
                msg: Option::from(String::from("Success")),
                data: Option::from(huy)
            })
        }
        Err(e) => {HttpResponse::InternalServerError().body(e.to_string())}
    }
}

