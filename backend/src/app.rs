use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    middleware,
    routing::get,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{admin, auth, bookings, movies, state::AppState};

pub fn build_app(state: AppState, frontend_origin: &str) -> anyhow::Result<Router> {
    let cors = CorsLayer::new()
        .allow_origin(frontend_origin.parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    Ok(Router::new()
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({ "ok": true })) }),
        )
        .nest("/api/auth", auth::routes())
        .nest("/api/movies", movies::routes())
        .nest(
            "/api/bookings",
            bookings::routes().route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::middleware::require_auth,
            )),
        )
        .nest(
            "/api/admin",
            admin::routes().route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::middleware::require_admin,
            )),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state))
}