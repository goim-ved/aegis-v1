use axum::{
    routing::post,
    Router,
};
use crate::state::AppState;

pub mod handlers;
pub mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pay", post(handlers::execute_transaction))
}
