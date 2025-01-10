use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCommunityData {
    pub description: Option<String>,
    pub price: u64,
    pub renewal_period: u64,
    pub need_wl: bool
}