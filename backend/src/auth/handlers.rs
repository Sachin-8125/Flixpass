use axum::{Json, extract::State};

use crate::{
    auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest},
        service,
    },
    error::ApiError,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    Ok(Json(service::register(state, input).await?))
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    Ok(Json(service::login(state, input).await?))
}