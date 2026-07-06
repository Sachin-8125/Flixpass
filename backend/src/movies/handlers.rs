use axum::{Json, extract::State};

use crate::{error::ApiError, movies::dto::MovieDto, movies::service, state::AppState};

pub async fn list_movies(State(state): State<AppState>) -> Result<Json<Vec<MovieDto>>, ApiError> {
    Ok(Json(service::list_movies(state).await?))
}