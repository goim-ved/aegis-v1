use sqlx::postgres::PgPool;
use std::sync::Arc;
use crate::chain::ChainClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub chain: Arc<ChainClient>,
}

