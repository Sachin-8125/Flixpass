use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::{
    models::{MovieRow, NewMovie},
    schema::movies,
};

pub fn list_active(conn: &mut PgConnection) -> QueryResult<Vec<MovieRow>> {
    movies::table
        .filter(movies::is_active.eq(true))
        .order(movies::created_at.asc())
        .select(MovieRow::as_select())
        .load(conn)
}

pub fn find_by_id(conn: &mut PgConnection, movie_id: &str) -> QueryResult<Option<MovieRow>> {
    movies::table
        .filter(movies::id.eq(movie_id))
        .select(MovieRow::as_select())
        .first(conn)
        .optional()
}

pub fn create(conn: &mut PgConnection, new_movie: &NewMovie) -> QueryResult<MovieRow> {
    diesel::insert_into(movies::table)
        .values(new_movie)
        .returning(MovieRow::as_returning())
        .get_result(conn)
}

pub fn soft_delete(conn: &mut PgConnection, movie_id: &str) -> QueryResult<usize> {
    diesel::update(movies::table.filter(movies::id.eq(movie_id)))
        .set(movies::is_active.eq(false))
        .execute(conn)
}