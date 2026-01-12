use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance_wei: String,
}

#[derive(Deserialize)]
pub struct FundRequest {
    pub wallet_address: String,
    pub amount_eth: String,
}

#[derive(Serialize)]
pub struct FundResponse {
    pub tx_hash: String,
    pub status: String,
}
