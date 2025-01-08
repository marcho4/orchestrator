use actix_web::{get, post, web, HttpResponse, Result as ActixResult};
use actix_web::cookie::time::macros::date;
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};
use actix_web::http::Error as HttpError;
use log::{error, info};
use serde_json::json;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, Code, EmailSend, PubCommunityInfo};
use crate::models::community::Community;
use crate::models::license::License;
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


#[post("/community/create")]
pub async fn create_community(data: web::Data<Mutex<Orchestrator>>, community: web::Json<Community>) -> HttpResponse {
    let orchestrator = data.lock().await;
    let community = community.into_inner();
    let api_name = community.api_name.clone();

    let api_name = match api_name {
        Some(name) => name,
        None => return HttpResponse::BadRequest().json(ApiResponse::<String> {
            msg: Some("API name is missing".to_string()),
            data: None,
        }),
    };

    let is_community_found = match orchestrator.client
        .get(format!("http://community_service:8003/community/{}", api_name))
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(api_response) = response.json::<ApiResponse<Community>>().await {
                api_response.data.is_some()
            } else {
                false
            }
        },
        Err(e) => {
            info!("Error while getting community: {}", api_name);
            return HttpResponse::BadRequest().json(ApiResponse::<String> {
                msg: Some(e.to_string()),
                data: None,
            });
        }
    };

    if is_community_found {
        info!("{}", format!("Community with name {} already exists", api_name.clone()));
        return HttpResponse::BadRequest().json(ApiResponse::<Community> {
            msg: Option::from(String::from("There is already a community with this name")),
            data: None
        })
    };

    info!("Generating code community {:?}", &community);
    let code = orchestrator.client
        .post("http://code_service:8005/code/generate")
        .json(&community)
        .send().await;

    let code = match code {
        Ok(code) => code.json::<ApiResponse<String>>().await,
        Err(e) => {
            error!("Error while generating code community: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                msg: Option::from("Error with code creation. Try again".to_string()),
                data: None
            })
        }
    };

    let parsed_code = match code {
        Ok(code) => code.data.unwrap(),
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<String> {
            msg: Option::from(String::from("Error with generating the code")),
            data: None
        })
    };


    let code = parsed_code.clone();
    let email_response = orchestrator.client
        .post("http://email_service:8006/email/send")
        .json(&EmailSend {
            receiver: community.owners_email,
            subject: String::from("Your activation code"),
            title: "Welcome to the dao.build".to_string(),
            code_type: "code to activate the community".to_string(),
            body: code,
        })
        .send().await;

    match email_response {
        Ok(response) => {
            if response.status().is_success() {
                return HttpResponse::Ok().json(ApiResponse::<String> {
                    msg: Option::from("Success".to_string()),
                    data: None
                })
            }
            HttpResponse::BadRequest().json(ApiResponse::<String> {
                msg: Option::from("Error sending email".to_string()),
                data: None
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<String> {
                msg: Option::from(e.to_string()),
                data: None
        })
    }

}

#[get("/{api_name}/members")]
pub async fn fetch_all_community_members(api_name: web::Path<String>, data: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let api_name = api_name.into_inner();
    let url = format!("http://license_service:8001/license/{api_name}/all");
    let response = data.lock().await.client.get(url).send().await.map_err(
        |_| ErrorInternalServerError("Error with request to license service")
    )?;
    let json = response.json::<ApiResponse<Vec<License>>>().await.map_err(
        |_| ErrorInternalServerError("No members")
    )?;
    if json.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
            msg: Some("Success".to_string()),
            data: Some(json.data.unwrap())
        }))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<License>> {
            msg: Some("No data was found".to_string()),
            data: Some(Vec::new())
        }))
    }
}

#[get("/{api_name}/wallets")]
pub async fn fetch_all_allowed_wallets(api_name: web::Path<String>, data: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    let url = format!("http://community_service:8003/community/allowed_wallets/{api_name}");
    let response = data.lock().await.client.get(url).send().await.map_err(
        |_| ErrorInternalServerError("Error with request to community service")
    )?;
    let json = response.json::<ApiResponse<Vec<String>>>().await.map_err(
        |_| ErrorInternalServerError("No licenses")
    )?;
    if json.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<String>> {
            msg: Some("Success".to_string()),
            data: Some(json.data.unwrap())
        }))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::<Vec<String>> {
            msg: Some("No data was found".to_string()),
            data: Some(Vec::new())
        }))
    }
}


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

#[post("/generate_transaction")]
pub async fn generate_transaction(data: web::Data<Mutex<Orchestrator>>) -> HttpResponse {
    unimplemented!()
}

#[get("/{api_name}/check/{wallet}")]
pub async fn check_wallet(data: web::Data<Mutex<Orchestrator>>, path: web::Path<(String, String)>) -> ActixResult<HttpResponse> {
    let orchestrator = data.lock().await;
    let (api_name, wallet) = path.into_inner();
    let url = format!("http://community_service:8003/community/{api_name}/check/{wallet}");
    let req = orchestrator.client.get(url).send().await.map_err(|e| {
        ErrorInternalServerError(format!("Error getting community data: {}", e))
    })?.json::<ApiResponse<bool>>().await.map_err(|e| {
        ErrorInternalServerError(format!("Error getting community data: {}", e))
    })?;
    if req.data.is_some() {
        Ok(HttpResponse::Ok().json(ApiResponse::<bool> {
            msg: Option::from(String::from("Success")),
            data: req.data
        }))
    } else {
        Ok(HttpResponse::BadRequest().json(ApiResponse::<bool> {
            msg: Option::from(format!("No such wallet")),
            data: None
        }))
    }
}
pub async fn process_payment(data: web::Data<Mutex<Orchestrator>>) -> ActixResult<HttpResponse> {
    // Check if user is allowed
    // Community data sent in request
    // requesting transaction
    // Sending transaction data back to user
    unimplemented!()
}