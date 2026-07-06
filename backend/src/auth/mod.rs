pub mod dto;
pub mod handlers;
pub mod jwt;
pub mod middleware;
pub mod password;
pub mod service;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}