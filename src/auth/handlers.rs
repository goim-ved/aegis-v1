use axum::{extract::State, http::StatusCode, Json, response::{IntoResponse, Response}};
use sqlx::Row;
use crate::state::AppState;
use super::models::{AuthRequest, AuthResponse};
use super::utils::{hash_password, verify_password, create_jwt};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let row = sqlx::query("SELECT password_hash, role FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(user_row) = row {
        let password_hash: String = user_row.get("password_hash");
        let role: String = user_row.get("role");

        if verify_password(&payload.password, &password_hash) {
            let token = create_jwt(&payload.username, &role)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed".to_string()))?;
            return Ok(Json(AuthResponse { token }));
        }
    }
    
    // Fallthrough to Unauthorized
    Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
}


pub async fn register_admin(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let password_hash = hash_password(&payload.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed".to_string()))?;

    sqlx::query("INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'admin')")
        .bind(&payload.username)
        .bind(password_hash)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
