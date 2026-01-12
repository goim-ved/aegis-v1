use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use crate::state::AppState;
use super::models::{BalanceResponse, FundRequest, FundResponse};

pub async fn get_balance(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let balance = state.chain.get_wallet_balance(&address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(BalanceResponse {
        address,
        balance_wei: balance,
    }))
}

pub async fn fund_wallet(
    State(state): State<AppState>,
    Json(payload): Json<FundRequest>,
) -> Result<Json<FundResponse>, (StatusCode, String)> {
    let tx_hash = state.chain.fund_wallet(&payload.wallet_address, &payload.amount_eth)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(FundResponse {
        tx_hash,
        status: "Funded".to_string(),
    }))
}
