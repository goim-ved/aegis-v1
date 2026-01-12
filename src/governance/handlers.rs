use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use crate::state::AppState;
use super::models::{SetLimitRequest, GovernanceResponse};

pub async fn set_limit(
    State(state): State<AppState>,
    Json(payload): Json<SetLimitRequest>,
) -> Result<Json<GovernanceResponse>, (StatusCode, String)> {
    let tx_hash = state.chain.set_agent_limit(
            &payload.rules_contract, 
            &payload.agent_address, 
            &payload.limit_eth
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(GovernanceResponse {
        tx_hash,
        status: "Limit Updated".to_string(),
    }))
}
