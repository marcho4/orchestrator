use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, Claims};
use crate::models::community::Community;
use crate::orchestrator::orchestrator::Orchestrator;

#[get("/get/ownerships/{wallet}")]
pub async fn get_ownerships(data: web::Data<Mutex<Orchestrator>>, wallet: web::Path<String>,
                            req: HttpRequest) -> HttpResponse {
    if let Some(claims) = req.extensions().get::<Claims>() {
        info!("{:?}", claims);

        let wallet = wallet.into_inner();

        if wallet != claims.wallet {
            return HttpResponse::Forbidden().finish();
        }

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
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<String> {
            msg: Some("Unauthorized".to_string()),
            data: None
        })
    }
}

