use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub wallet: String,
    pub memberships: Vec<String>,
    pub ownerships: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
    pub jwt: String
}