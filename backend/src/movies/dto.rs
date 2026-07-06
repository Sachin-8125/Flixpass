use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MovieDto {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub rating: String,
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
    pub synopsis: String,
    #[serde(rename = "posterTone")]
    pub poster_tone: Option<String>,
    pub showtimes: Vec<ShowtimeDto>,
}

#[derive(Debug, Serialize)]
pub struct ShowtimeDto {
    pub id: String,
    #[serde(rename = "startsAt")]
    pub starts_at: DateTime<Utc>,
    pub screen: String,
    #[serde(rename = "priceCents")]
    pub price_cents: i32,
    #[serde(rename = "totalSeats")]
    pub total_seats: i32,
    #[serde(rename = "bookedSeats")]
    pub booked_seats: Vec<i32>,
}