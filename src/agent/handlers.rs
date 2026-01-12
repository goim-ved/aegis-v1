use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use crate::state::AppState;
use super::models::{TransactionRequest, TransactionResponse};

pub async fn execute_transaction(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, (StatusCode, String)> {
    
    let tx_hash = if let Some(token_addr) = &payload.token_address {
        // ERC20 Flow
        state.chain.execute_erc20(
            &payload.wallet_address,
            token_addr,
            &payload.target_address,
            &payload.amount
        ).await
    } else {
        // Native ETH Flow
        state.chain.execute_native(
            &payload.wallet_address,
            &payload.target_address,
            &payload.amount
        ).await
    };

    let tx_hash = tx_hash.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TransactionResponse {
        tx_hash,
        status: "Transaction Sent".to_string(),
    }))
}
