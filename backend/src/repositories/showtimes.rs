use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::{
    models::{NewShowtime, ShowtimeRow},
    schema::showtimes,
};

pub fn list_for_movie_ids(
    conn: &mut PgConnection,
    movie_ids: &[String],
) -> QueryResult<Vec<ShowtimeRow>> {
    if movie_ids.is_empty() {
        return Ok(Vec::new());
    }

    showtimes::table
        .filter(showtimes::movie_id.eq_any(movie_ids))
        .order(showtimes::starts_at.asc())
        .select(ShowtimeRow::as_select())
        .load(conn)
}

pub fn find_for_update(
    conn: &mut PgConnection,
    showtime_id: &str,
) -> QueryResult<Option<ShowtimeRow>> {
    showtimes::table
        .filter(showtimes::id.eq(showtime_id))
        .for_update()
        .select(ShowtimeRow::as_select())
        .first(conn)
        .optional()
}

pub fn create(conn: &mut PgConnection, new_showtime: &NewShowtime) -> QueryResult<ShowtimeRow> {
    diesel::insert_into(showtimes::table)
        .values(new_showtime)
        .returning(ShowtimeRow::as_returning())
        .get_result(conn)
}

pub fn list_for_movie_id(conn: &mut PgConnection, movie_id: &str) -> QueryResult<Vec<ShowtimeRow>> {
    showtimes::table
        .filter(showtimes::movie_id.eq(movie_id))
        .order(showtimes::starts_at.asc())
        .select(ShowtimeRow::as_select())
        .load(conn)
}