use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::cookie::{Cookie, Expiration, SameSite};
use actix_web::cookie::time::OffsetDateTime;
use tokio::sync::Mutex;
use crate::models::api_models::{ApiResponse, LoginData};
use crate::orchestrator::orchestrator::Orchestrator;

#[post("/login")]
pub async fn login(data: web::Data<Mutex<Orchestrator>>, req_data: web::Json<LoginData>) -> HttpResponse {
    let login_data = req_data.into_inner();
    data.lock().await.process_login(login_data.wallet, login_data.signature).await
}

#[post("/logout")]
pub async fn logout() -> HttpResponse {
    let expired_cookie = Cookie::build("token", "")
        .path("/")
        .same_site(SameSite::None)
        .expires(Expiration::from(OffsetDateTime::UNIX_EPOCH))
        .finish();

    HttpResponse::Ok()
        .cookie(expired_cookie)
        .json(ApiResponse::<String> {
            msg: Option::from(String::from("success")),
            data: None
        })
}

#[get("/auth/request_nonce/{wallet}")]
pub async fn request_nonce(data: web::Data<Mutex<Orchestrator>>, wallet: web::Path<String>) -> HttpResponse {
    let wallet = wallet.into_inner();
    let mut lock = data.lock().await;
    lock.request_nonce(wallet).await
}

#[get("/auth/session")]
pub async fn get_session(data: web::Data<Mutex<Orchestrator>>, req: HttpRequest) -> HttpResponse {
    let cookie = req.cookie("token");
    if let Some(cookie) = cookie {
        let lock = data.lock().await;
        lock.get_session(cookie.value().to_string()).await
    } else {
        HttpResponse::Unauthorized().json(
            ApiResponse::<String> {
                msg: Option::from(String::from("Token Not Found")),
                data: None
            }
        )
    }
}
