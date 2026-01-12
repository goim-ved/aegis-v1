pub mod models;
pub mod handlers;

use axum::{
    routing::post,
    Router,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register_entity))
        .route("/mint", post(handlers::mint_token))
        .route("/entities", axum::routing::get(handlers::list_entities))
}

