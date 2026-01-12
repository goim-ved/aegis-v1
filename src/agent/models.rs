use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub wallet_address: String,
    pub target_address: String,
    pub amount: String,
    pub token_address: Option<String>,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub tx_hash: String,
    pub status: String,
}
