use serde::{Deserialize, Serialize};

use crate::types::BookingStatus;

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    #[serde(rename = "showtimeId")]
    pub showtime_id: String,
    pub seats: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct BookingDto {
    pub id: String,
    #[serde(rename = "movieTitle")]
    pub movie_title: String,
    pub seats: Vec<i32>,
    #[serde(rename = "totalCents")]
    pub total_cents: i32,
    pub status: BookingStatus,
}