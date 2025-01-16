use std::collections::HashMap;
use std::str::FromStr;
use actix_web::{post, web, HttpResponse, Result as ActixResult};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use chrono::Utc;
use log::info;
use reqwest;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::models::api_models::{ApiResponse, EmailSend};
use crate::models::payment_details::PaymentDetails;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status_client_types::{UiTransactionEncoding, UiTransactionStatusMeta, UiTransactionTokenBalance};
use solana_transaction_status_client_types::option_serializer::OptionSerializer;
use crate::models::community::Community;
use crate::models::license::License;

pub const RPC: &str = "https://mainnet.helius-rpc.com/?api-key=57609837-4d67-42dc-8ee6-1307bb7df433";
const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
const EPSILON: f64 = 0.000001;

pub fn generate_license_key() -> String {
    let rng = rand::thread_rng();
    let rand_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect();
    println!("{}", rand_string);
    format!("{}-{}-{}", &rand_string[0..4], &rand_string[4..8], &rand_string[8..12])
}
fn find_usdt_transfer(transaction_meta: &UiTransactionStatusMeta) -> Option<(String, String, f64)> {
    if let (OptionSerializer::Some(pre_balances), OptionSerializer::Some(post_balances)) =
        (&transaction_meta.pre_token_balances, &transaction_meta.post_token_balances) {

        // Создаем мапы баланс -> владелец для USDT
        let pre_map: HashMap<String, f64> = pre_balances
            .iter()
            .filter(|balance| balance.mint == USDT.to_string())
            .map(|balance| (
                    balance.owner.clone().unwrap(),
                    balance.ui_token_amount.ui_amount.unwrap_or(0.0)
            ))
            .collect();

        info!("{:?}", pre_map);

        let post_map: HashMap<String, f64> = post_balances
            .iter()
            .filter(|balance| balance.mint == USDT)
            .map(|balance| (
                balance.owner.clone().unwrap(),
                balance.ui_token_amount.ui_amount.unwrap_or(0.0)
            ))
            .collect();


        info!("{:?}", post_map);

        let mut sender = None;
        let mut receiver = None;
        let mut transfer_amount = 0.0;

        // Ищем отправителя (отрицательная разница)
        for (wallet, pre_amount) in &pre_map {
            if let Some(post_amount) = post_map.get(wallet) {
                let diff = post_amount - pre_amount;
                if diff < 0.0 {
                    sender = Some(wallet.clone());
                    transfer_amount = diff.abs();
                }
            }
        }
        info!("Transfer amount: {:?}", transfer_amount);
        // Ищем получателя (положительная разница)
        for (wallet, pre_amount) in &pre_map {
            if let Some(post_amount) = post_map.get(wallet) {
                let diff = post_amount - pre_amount;
                info!("Difference: {:?}", diff);
                if diff > 0.0 && (diff.abs() - transfer_amount) < EPSILON {
                    receiver = Some(wallet.clone());
                }
            }
        }
        info!("Receiver: {:?}", receiver);
        info!("Sender: {:?}", sender);
        // Если нашли и отправителя и получателя, возвращаем их
        if let (Some(sender_wallet), Some(receiver_wallet)) = (sender, receiver) {
            return Some((sender_wallet, receiver_wallet, transfer_amount));
        }
    }

    None
}
#[post("/process_payment")]
pub async fn process_payment(details: web::Json<PaymentDetails>) -> ActixResult<HttpResponse> {
    let payment_details = details.into_inner();

    // Получение данных о комьюнити, для которого происходит оплата
    let comm_url = &format!("http://community_service:8003/community/{}", payment_details.api_name);
    let community = reqwest::get(comm_url).await
        .map_err(|e| {ErrorBadRequest(e)})?
        .json::<ApiResponse<Community>>().await
        .map_err(|e| ErrorInternalServerError(e))?.data
        .ok_or(ErrorNotFound("Community not found"))?;

    // Добавляю необходимые данные в переменные
    let price = community.price;
    let collect_wallet = community.collect_wallet;

    // Обработка сигнатуры транзакции
    let signature = Signature::from_str(&payment_details.signature)
        .map_err(|e| {ErrorBadRequest(e)})?;

    // Обработка данных из блокчейна для валидации транзакции
    let rpc_client = RpcClient::new_with_commitment(RPC.to_string(), CommitmentConfig::confirmed());
    let parsed_tx = rpc_client.get_transaction(&signature, UiTransactionEncoding::Json).await
        .map_err(|e| {ErrorBadRequest(e)})?.transaction.meta.unwrap();

    // Обработка некорректной транзакции
    if parsed_tx.err.is_some() {
        return Err(ErrorBadRequest("Transaction failed"))
    }

    if let Some((sender, receiver, amount)) = find_usdt_transfer(&parsed_tx) {
        info!("Amount sent from {} to {}: {}", sender, receiver, amount);
        // Обработка случая, когда получатель денег не сходится с кошельком сбора денег из апи
        if receiver != collect_wallet {
            return Err(ErrorBadRequest("Transaction was sent to the wrong wallet."));
        }

        // Обработка случая, если заплатили меньше чем надо
        if (amount - price as f64) > EPSILON {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<String> {
                msg: Some("Amount paid is lower than community's price".to_string()),
                data: None
            }))
        }

        // Generating license
        let lic_url = "http://license_service:8001/license/create";

        let license = License {
            id: None,
            user_id: None,
            license: generate_license_key(),
            wallet: sender.to_string(),
            community: payment_details.community_name,
            api_name: Some(payment_details.api_name),
            invite: None,
            expiration: None,
            activated: false,
            created_at: Utc::now().timestamp(),
        };

        // Обращение к микросервису лицензий для создания одной из них
        let r = reqwest::ClientBuilder::new().build().unwrap().post(lic_url).json(&license)
            .send().await
            .map_err(|e| ErrorBadRequest("Bad request to license service"))?;

        // Обработка некорректного создания лицензии
        if !r.status().is_success() {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<String> {
                msg: Some("License haven't been created.".to_string()),
                data: None
            }))
        }

        // Добавление использованной лицензии в бд, чтобы избежать повторного использования
        // Отправка email на почту челика с лицензионным ключом для активации
        let email_url = "http://email_service:8006/email/send";
        let email_content = EmailSend {
            receiver: payment_details.email,
            subject: format!("Your License Key for {}", community.name),
            title: format!("Welcome to {}", community.name),
            code_type: "license".to_string(),
            body: license.license.to_string(),
        };
        info!("Email content: {:?}", email_content);

        let email_response = reqwest::ClientBuilder::new().build().unwrap().post(email_url).json(&email_content)
            .send().await;
        if email_response.is_err() {info!("Email was not send")}

        Ok(HttpResponse::Ok().json(ApiResponse::<License> {
            msg: Some("License successfully created.".to_string()),
            data: Some(license),
        }))
    } else {
        // Обработка случая, если транзакция не имеет трансфера в USDT
        Err(ErrorInternalServerError("Transaction doesn't have USDT transfers"))
    }
}