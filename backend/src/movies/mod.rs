pub mod dto;
pub mod handlers;
pub mod service;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(handlers::list_movies))
}