pub mod handlers;
pub mod service;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/movies/demo", post(handlers::create_demo_movie))
        .route("/movies/:id", axum::routing::delete(handlers::delete_movie))
}