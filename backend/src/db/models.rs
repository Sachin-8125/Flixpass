use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use crate::db::schema::{bookings, movies, showtimes, users};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = movies)]
pub struct MovieRow {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub rating: String,
    pub duration_minutes: i32,
    pub synopsis: String,
    pub poster_tone: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub rating: String,
    pub duration_minutes: i32,
    pub synopsis: String,
    pub poster_tone: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = showtimes)]
pub struct ShowtimeRow {
    pub id: String,
    pub movie_id: String,
    pub starts_at: DateTime<Utc>,
    pub screen: String,
    pub price_cents: i32,
    pub total_seats: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = showtimes)]
pub struct NewShowtime {
    pub id: String,
    pub movie_id: String,
    pub starts_at: DateTime<Utc>,
    pub screen: String,
    pub price_cents: i32,
    pub total_seats: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = bookings)]
pub struct BookingRow {
    pub id: String,
    pub user_id: String,
    pub showtime_id: String,
    pub seats: Vec<i32>,
    pub total_cents: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = bookings)]
pub struct NewBooking {
    pub id: String,
    pub user_id: String,
    pub showtime_id: String,
    pub seats: Vec<i32>,
    pub total_cents: i32,
    pub status: String,
}