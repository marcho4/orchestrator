use serde::{Deserialize, Serialize};
use spl_token::solana_program::instruction::AccountMeta;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SerializedInstruction {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub data: String,
}