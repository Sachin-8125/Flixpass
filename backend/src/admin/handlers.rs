use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{admin::service, error::ApiError, movies::dto::MovieDto, state::AppState};

pub async fn create_demo_movie(State(state): State<AppState>) -> Result<Json<MovieDto>, ApiError> {
    Ok(Json(service::create_demo_movie(state).await?))
}

pub async fn delete_movie(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    service::delete_movie(state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}