use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub logo: String,
    pub social: String,
    pub group_id: String,
    pub renewal_period: i32,
    pub owners_wallet: String,
    pub owners_email: String,
    pub need_wl: bool,
    pub allowed_wallets: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub license: String,
    pub wallet: String,
    pub community: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    pub activated: bool,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub wallet: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub wallet: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
    pub jwt: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub wallet: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSend {
    pub receiver: String,
    pub subject: String,
    pub title: String,
    pub code_type: String,
    pub body: String
}