use axum::{Json, extract::State};

use crate::{
    auth::middleware::Principal,
    bookings::{
        dto::{BookingDto, CreateBookingRequest},
        service,
    },
    error::ApiError,
    state::AppState,
};

pub async fn create_booking(
    State(state): State<AppState>,
    principal: Principal,
    Json(input): Json<CreateBookingRequest>,
) -> Result<Json<BookingDto>, ApiError> {
    Ok(Json(
        service::create_booking(state, principal, input).await?,
    ))
}

pub async fn my_bookings(
    State(state): State<AppState>,
    principal: Principal,
) -> Result<Json<Vec<BookingDto>>, ApiError> {
    Ok(Json(service::my_bookings(state, principal).await?))
}