use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct RegisterEntityRequest {
    pub hash_id: String,
    pub jurisdiction: String,
    pub kyc_level: i16, // using i16 for SMALLINT
}

#[derive(Debug, Serialize, FromRow)]
pub struct LegalEntity {
    pub id: i32,
    pub hash_id: String,
    pub jurisdiction: String,
    pub kyc_level: i16,
    pub on_chain_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
