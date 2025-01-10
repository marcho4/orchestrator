use actix_web::{post, web, HttpResponse, Result as ActixResult};
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};
use log::info;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::instruction_data::InstructionData;
use crate::models::serialized_instruction::SerializedInstruction;
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/generate_transaction")]
pub async fn generate_transaction(data: web::Data<Mutex<Orchestrator>>, transaction_data: web::Json<InstructionData>) -> ActixResult<HttpResponse> {
    let orchestrator = data.lock().await;
    let url = "http://solana_service:8010/generate_instruction".to_string();
    let instruction = transaction_data.into_inner();
    let request = orchestrator.client.post(url)
        .json(&instruction)
        .send().await.map_err(|e| ErrorBadGateway(e))?;

    info!("{}", format!("{:?}", request));

    let data = request.json::<ApiResponse<SerializedInstruction>>().await;

    match data {
        Ok(data) => Ok(HttpResponse::Ok().json(&data)),
        Err(e) => Err(ErrorInternalServerError(e))
    }
}