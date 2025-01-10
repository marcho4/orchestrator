use actix_web::{post, web, HttpResponse, Result as ActixResult};
use serde_json::json;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, Wallets};
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/{api_name}/add_wallets_to_wl")]
pub async fn add_wallets_to_wl(
    orchestrator: web::Data<Mutex<Orchestrator>>,
    data: web::Json<Wallets>,
    api_name: web::Path<String>
) -> ActixResult<HttpResponse> {
    let orchestrator = orchestrator.lock().await;
    let api_name = api_name.into_inner();
    let url = format!("http://community_service:8003/community/allow/{api_name}");

    let wallets = data.into_inner().wallets;

    // Используем futures::stream для параллельной обработки с ограничением
    use futures::StreamExt;

    let results = futures::stream::iter(wallets)
        .map(|wallet| {
            let url = url.clone();
            let client = &orchestrator.client;
            async move {
                client.post(&url)
                    .json(&json!({"wallet": wallet}))
                    .send()
                    .await
                    .is_ok()
            }
        })
        .buffer_unordered(10) // Максимум 10 параллельных запросов
        .collect::<Vec<bool>>()
        .await;

    let successful = results.iter().filter(|&&success| success).count();

    Ok(HttpResponse::Ok().json(ApiResponse::<usize> {
        msg: Some("Success".to_string()),
        data: Some(successful)
    }))
}