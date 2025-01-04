use std::error::Error;
use std::str::FromStr;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::Duration;
use actix_web::HttpResponse;
use rand::distributions::Alphanumeric;
use rand::Rng;
use redis::Commands;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use crate::orchestrator::orchestrator::Orchestrator;
use crate::models::api_models::*;
use chrono::{Utc};
use log::info;

impl Orchestrator {
    fn verify_solana_signature(pubkey: &Pubkey, message: &[u8], signature: &Signature) -> bool {
        signature.verify(pubkey.as_ref(), message)
    }
    pub async fn get_memberships(&self, wallet: String) -> Result<Vec<License>, Box<dyn Error>> {
        let memberships_url = format!("http://license_service:8001/license/all/{}", wallet);

        let response_text = self.client
            .get(&memberships_url)
            .send()
            .await?
            .text()
            .await?;

        let api_response: ApiResponse<Vec<License>> =
            serde_json::from_str(&response_text)?;

        Ok(api_response.data.unwrap_or_default())
    }
    pub async fn get_ownerships(&self, wallet: String) -> Result<Vec<Community>, Box<dyn Error>> {
        let communities_url = format!("http://community_service:8003/community/all/{}", wallet);

        let response_text = self.client
            .get(&communities_url)
            .send()
            .await?
            .text()
            .await?;

        let api_response: ApiResponse<Vec<Community>> =
            serde_json::from_str(&response_text)?;

        Ok(api_response.data.unwrap_or_default())
    }
    pub async fn get_jwt(&self, jwt_data: JwtClaims) -> Result<JwtResponse, Box<dyn Error>> {
        let jwt_url = "http://jwt_service:8002/jwt/generate";
        let request = self.client.post(jwt_url).json(&jwt_data).send().await?.json::<JwtResponse>().await?;
        Ok(request)
    }
    pub async fn process_login(&mut self, wallet: String, signature: String) -> HttpResponse {
        let nonce: String = match self.redis_client.get(format!("nonce:{}", &wallet)) {
            Ok(nonce) => nonce,
            Err(_) => return HttpResponse::InternalServerError().body("nonce is not found"),
        };

        let pubkey = match Pubkey::from_str(&wallet) {
            Ok(pu) => pu,
            Err(e) => {
                log::error!("Invalid wallet format: {}", e);
                return HttpResponse::BadRequest().body("Invalid wallet format")
            }
        };

        let signature = match Signature::from_str(&signature) {
            Ok(sign) => sign,
            Err(e) => {
                log::error!("Invalid signature: {}", e);
                return HttpResponse::BadRequest().body("Invalid signature format")
            }
        };

        let is_valid = Orchestrator::verify_solana_signature(&pubkey, nonce.as_bytes(), &signature);

        if !is_valid {
            return HttpResponse::Unauthorized().finish();
        }

        let jwt_data = JwtClaims {wallet};

        match self.get_jwt(jwt_data).await {
            Ok(jwt) => {
                // Создание cookie
                let cookie = Cookie::build("token", jwt.jwt.clone())
                    .path("/")
                    .max_age(Duration::seconds(3600))
                    .same_site(SameSite::None)
                    .finish();

                HttpResponse::Ok()
                    .cookie(cookie)
                    .json(ApiResponse::<String> {
                        msg: Option::from(String::from("Success")),
                        data: Option::from(jwt.jwt),
                    })
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
        }
    }

    pub async fn request_nonce(&mut self, wallet: String) -> HttpResponse {
        let nonce = Orchestrator::generate_nonce();
        self.redis_client.set_ex::<String, String, String>(format!("nonce:{}", wallet), nonce.clone(), 300).expect("TODO: panic message");

        HttpResponse::Ok().json(ApiResponse::<String> {
            msg: Option::from(String::from("Success")),
            data: Option::from(nonce),
        })
    }

    fn generate_nonce() -> String {
        let nonce: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32) // 32 символа
            .map(char::from)
            .collect();
        nonce
    }

    pub async fn get_session(&self, jwt: String) -> HttpResponse {
        let jwt_url = "http://jwt_service:8002/jwt/decode";
        let jwt_data = JwtResponse {jwt};

        let request = self.client.post(jwt_url).json(&jwt_data);
        info!("{:?}", request);
        let res = request.send().await;

        match res {
            Ok(response) => {
                let data = match response.json::<ApiResponse<Claims>>().await {
                    Ok(data) => {
                        let claims = match data.data {
                            Some(claims) => claims,
                            None => {
                                return HttpResponse::Unauthorized().json(ApiResponse::<String> {
                                    msg: Option::from("Wrong JWT".to_string()),
                                    data: data.msg
                                });
                            }
                        };
                        claims
                    },
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(
                        ApiResponse::<String> {msg: Option::from(e.to_string()), data: None}
                    )}
                };
                // Проверка на просроченность jwt токена
                if data.exp < Utc::now().timestamp() {
                    return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                        msg: Option::from("Your JWT has been expired".to_string()),
                        data: None,
                    })
                };

                HttpResponse::Ok().json(ApiResponse::<String> {
                    msg: Option::from("Success".to_string()),
                    data: Option::from(data.wallet)
                })

            },
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<String> {
                msg: Option::from(e.to_string()),
                data: None
            }),
        }

    }
}