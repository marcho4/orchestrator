use actix_web::{post, web, HttpResponse, Result as ActixResult};
use actix_web::error::{ErrorBadGateway, ErrorBadRequest, ErrorInternalServerError};
use log::info;
use tokio::sync::Mutex;
use crate::models::api_models::ApiResponse;
use crate::models::community::Community;
use crate::models::instruction_data::InstructionData;
use crate::models::serialized_instruction::SerializedInstruction;
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/generate_transaction")]
pub async fn generate_transaction(data: web::Data<Mutex<Orchestrator>>, transaction_data: web::Json<InstructionData>) -> ActixResult<HttpResponse> {
    let orchestrator = data.lock().await;
    let instruction = transaction_data.into_inner();
    let wallet = instruction.sender.clone();
    let api_name = instruction.api_name.clone();

    // Getting community wl status
    let community_data = orchestrator.client.get(format!("http://community_service:8003/community/{api_name}")).send().await
        .map_err(|_| ErrorInternalServerError("Bad request to community microservice"))?;
    let community = community_data.json::<ApiResponse<Community>>().await.map_err(|e| ErrorInternalServerError("Can't process community json data"))?;

    // If communtiy doesn't exist
    if community.data.is_none() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<String> {
            msg: Some(format!("Community does not exist")),
            data: None,
        }));
    };

    // Checking wallet if wl mode is enabled
    if community.data.unwrap().need_wl {
        let url = format!("http://community_service:8003/community/{api_name}/check/{wallet}");
        let req = orchestrator.client.get(url).send().await.map_err(|e| {
            ErrorInternalServerError(format!("Error getting community data: {}", e))
        })?.json::<ApiResponse<bool>>().await.map_err(|e| {
            ErrorInternalServerError(format!("Error getting community data: {}", e))
        })?;

        // Если кошелька нет в вайтлисте, то
        if req.data.is_none() || (req.data.is_some() && req.data.unwrap() == false) {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<bool> {
                msg: Option::from("Wallet is not allowed to pay".to_string()),
                data: None
            }));
        };
    }


    let url = "http://solana_service:8010/generate_instruction".to_string();
    let request = orchestrator.client.post(url)
        .json(&instruction)
        .send().await.map_err(|e| ErrorBadGateway(e))?;
    info!("Generated transaction with data: {:?}", instruction);
    let data = request.json::<ApiResponse<SerializedInstruction>>().await;
    info!("Transaction data: {:?}", data);
    match data {
        Ok(data) => Ok(HttpResponse::Ok().json(&data)),
        Err(e) => Err(ErrorInternalServerError(e))
    }
}