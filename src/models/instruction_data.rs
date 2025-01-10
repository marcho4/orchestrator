use serde::{Deserialize, Serialize};
use crate::models::token::Token;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructionData {
    pub amount: u64,
    pub sender: String,
    pub receiver: String,
    pub token: Token
}