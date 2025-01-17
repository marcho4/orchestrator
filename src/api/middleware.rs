use std::future::{ready, Ready};
use std::rc::Rc;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use futures_util::future::LocalBoxFuture;
use log::{error, info};
use reqwest::Client;
use crate::models::api_models::{ApiResponse, Claims, JwtResponse};


pub struct AuthMiddleware {
    client: Client,
}
impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware {
            client: Client::new(),
        }
    }
}
// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SetUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SetUserMiddleware { service: Rc::new(service), client: self.client.clone() }))
    }
}

pub struct SetUserMiddleware<S> {
    service: Rc<S>,
    client: Client
}

impl<S, B> Service<ServiceRequest> for SetUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let client = self.client.clone();

        Box::pin(async move {
            let jwt = req.request().cookie("token");

            if let Some(jwt_token) = jwt {
                match decode_jwt(jwt_token.value().to_string(), &client).await {
                    Ok(claims) => {
                        info!("logging claims from middleware: {:?}", claims);
                        req.extensions_mut().insert(claims);
                    },
                    Err(e)=> {error!("{}", e)}
                }
            }

            // Если нет куки - ничего не ставим в req extenstions
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}

async fn decode_jwt(jwt: String, client: &Client) -> Result<Claims, Error> {
    let jwt_json = JwtResponse { jwt };
    info!("jwt string from middleware: {:?}", jwt_json);
    let resp = client.post("http://jwt_service:8002/jwt/decode")
        .json(&jwt_json).send().await;
    if resp.is_err() {
        return Err(ErrorUnauthorized("invalid response from jwt service"));
    };
    let resp = resp.unwrap();
    let data = resp.json::<ApiResponse<Claims>>().await;
    if data.is_err() {
        return Err(ErrorBadRequest("Can not decode jwt service response"));
    };
    info!("logging claims from middleware: {:?}", data);
    data.unwrap().data.ok_or_else(|| ErrorUnauthorized("JWT decode error"))
}