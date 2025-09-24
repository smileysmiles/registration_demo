use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{player::get_player, register::register},
    state::AppState,
};

/// Builds all application routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/register", post(register))
        .route("/player/{id}", get(get_player)) // <-- fixed for axum 0.8
        .with_state(state)
}
