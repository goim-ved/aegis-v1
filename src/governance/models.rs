use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetLimitRequest {
    pub rules_contract: String,
    pub agent_address: String,
    pub limit_eth: String,
}

#[derive(Serialize)]
pub struct GovernanceResponse {
    pub tx_hash: String,
    pub status: String,
}
