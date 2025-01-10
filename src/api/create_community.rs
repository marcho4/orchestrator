use actix_web::{post, web, HttpResponse};
use log::{error, info};
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, EmailSend};
use crate::models::community::Community;
use crate::orchestrator::orchestrator::Orchestrator;

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
