use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
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
pub struct Wallets {
    pub wallets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSend {
    pub receiver: String,
    pub subject: String,
    pub title: String,
    pub code_type: String,
    pub body: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubCommunityInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub logo: String,
    pub social: String,
    pub price: i32,
    pub renewal_period: i32,
    pub collect_wallet: String,
    pub need_wl: bool,
    pub plan: String,
}