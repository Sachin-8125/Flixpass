use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::{
    models::{BookingRow, NewBooking},
    schema::{bookings, movies, showtimes},
};
use crate::types::BookingStatus;

pub fn create(conn: &mut PgConnection, new_booking: &NewBooking) -> QueryResult<BookingRow> {
    diesel::insert_into(bookings::table)
        .values(new_booking)
        .returning(BookingRow::as_returning())
        .get_result(conn)
}

pub fn list_confirmed_for_showtime(
    conn: &mut PgConnection,
    showtime_id: &str,
) -> QueryResult<Vec<BookingRow>> {
    bookings::table
        .filter(bookings::showtime_id.eq(showtime_id))
        .filter(bookings::status.eq(BookingStatus::Confirmed.as_db()))
        .select(BookingRow::as_select())
        .load(conn)
}

pub fn list_confirmed_for_showtime_ids(
    conn: &mut PgConnection,
    showtime_ids: &[String],
) -> QueryResult<Vec<BookingRow>> {
    if showtime_ids.is_empty() {
        return Ok(Vec::new());
    }

    bookings::table
        .filter(bookings::showtime_id.eq_any(showtime_ids))
        .filter(bookings::status.eq(BookingStatus::Confirmed.as_db()))
        .select(BookingRow::as_select())
        .load(conn)
}

pub fn list_for_user_with_movie(
    conn: &mut PgConnection,
    user_id: &str,
) -> QueryResult<Vec<(String, String, Vec<i32>, i32, String)>> {
    bookings::table
        .inner_join(showtimes::table.inner_join(movies::table))
        .filter(bookings::user_id.eq(user_id))
        .order(bookings::created_at.desc())
        .select((
            bookings::id,
            movies::title,
            bookings::seats,
            bookings::total_cents,
            bookings::status,
        ))
        .load::<(String, String, Vec<i32>, i32, String)>(conn)
}
