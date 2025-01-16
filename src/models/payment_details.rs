use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PaymentDetails {
    pub signature: String,
    pub api_name: String,
    pub email: String,
    pub community_name: String,
}