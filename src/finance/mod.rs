use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState;

pub mod handlers;
pub mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/balance/:address", get(handlers::get_balance))
        .route("/fund", post(handlers::fund_wallet))
}
