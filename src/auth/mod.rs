pub mod models;
pub mod handlers;
pub mod utils;

use axum::{
    routing::post,
    Router,
};
use crate::state::AppState;
use axum_extra::headers::{Authorization, authorization::Bearer};
use axum_extra::TypedHeader;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, http::StatusCode};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register_admin))
}

// Extractor for JWT
#[derive(Clone)]
pub struct Claims(pub models::Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Axum 0.7 uses `from_request` for items that consume the body, but headers don't.
        // However, TypedHeader in axum-extra might still need FromRequestParts.
        // The error said `from_request_parts` not found for TypedHeader, suggesting it might be `from_request` or similar.
        // Actually, for axum-extra 0.9 + axum 0.7:
        // TypedHeader implements FromRequestParts.
        // But let's simplify.
        
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Missing Bearer Token".to_string()))?;

        let token_data = utils::decode_jwt(bearer.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Token".to_string()))?;

        Ok(Claims(token_data))
    }
}
