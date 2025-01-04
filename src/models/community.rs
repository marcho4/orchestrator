use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub price: i32,
    pub renewal_period: i32,
    pub owners_wallet: String,
    pub collect_wallet: String,
    pub owners_email: String,
    pub need_wl: bool,
    pub allowed_wallets: Vec<String>,
    pub plan: String,
}