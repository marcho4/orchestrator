use actix_web::{get, post, web, HttpResponse};
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, Code, Community, EmailSend, License};
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
    // check if community exists with this name
    // creating code
    // sending code to an email
    // return success
    let orchestrator = data.lock().await;
    let community = community.into_inner();
    let api_name = community.api_name.clone();
    let is_community_found = match orchestrator.client.get(format!("http://localhost:8003/community/{}", api_name.unwrap())).send().await {
        Ok(response) => response.json::<ApiResponse<Community>>().await.unwrap().data.is_some(),
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<String> {
            msg: Option::from(e.to_string()),
            data: None
        })
    };

    if is_community_found {
        return HttpResponse::BadRequest().json(ApiResponse::<Community> {
            msg: Option::from(String::from("There is already a community with this name")),
            data: None
        })
    };

    let code = orchestrator.client.post("http://localhost:8005/code/generate")
        .json(&community).send().await;

    let code = match code {
        Ok(code) => code.json::<Code>().await,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<String> {
            msg: Option::from(e.to_string()),
            data: None
        })
    };

    let parsed_code = match code {
        Ok(code) => code,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<String> {
            msg: Option::from(String::from("Error with generating the code")),
            data: None
        })
    };
    let code = parsed_code.code.clone();
    let email_response = orchestrator.client
        .post("http://localhost:8006/email/send")
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
                msg: Option::from(format!("Error sending email")),
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