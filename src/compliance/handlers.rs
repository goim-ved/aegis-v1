use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::state::AppState;
use super::models::{RegisterEntityRequest, LegalEntity};
use serde::Deserialize;

pub async fn register_entity(
    State(state): State<AppState>,
    Json(payload): Json<RegisterEntityRequest>,
) -> Result<Json<LegalEntity>, (StatusCode, String)> {
    if payload.kyc_level <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Invalid KYC Level".to_string()));
    }

    let entity = sqlx::query_as::<_, LegalEntity>(
        r#"
        INSERT INTO legal_entities (hash_id, jurisdiction, kyc_level)
        VALUES ($1, $2, $3)
        RETURNING id, hash_id, jurisdiction, kyc_level, on_chain_id, created_at
        "#
    )
    .bind(payload.hash_id)
    .bind(payload.jurisdiction)
    .bind(payload.kyc_level)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(entity))
}

#[derive(Deserialize)]
pub struct MintRequest {
    pub wallet_address: String,
    pub uri: String,
}

use crate::auth::Claims;

pub async fn mint_token(
    State(state): State<AppState>,
    _: Claims, // Requires valid JWT
    Json(payload): Json<MintRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    let tx_hash = state.chain.mint(&payload.wallet_address, &payload.uri)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Minting failed: {}", e)))?;

    Ok(Json(tx_hash))
}

pub async fn list_entities(
    State(state): State<AppState>,
    _: Claims,
) -> Result<Json<Vec<LegalEntity>>, (StatusCode, String)> {
    let entities = sqlx::query_as::<_, LegalEntity>(
        "SELECT * FROM legal_entities ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(entities))
}


