pub mod dto;
pub mod handlers;
pub mod service;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_booking))
        .route("/me", get(handlers::my_bookings))
}