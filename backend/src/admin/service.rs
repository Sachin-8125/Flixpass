use chrono::{Duration, Utc};
use diesel::Connection;

use crate::{
    db::models::{NewMovie, NewShowtime},
    error::ApiError,
    movies::{dto::MovieDto, service as movie_service},
    repositories::{bookings, movies, showtimes},
    state::AppState,
};

pub async fn create_demo_movie(state: AppState) -> Result<MovieDto, ApiError> {
    let pool = state.db.clone();

    tokio::task::spawn_blocking(move || -> Result<MovieDto, ApiError> {
        let mut conn = pool.get()?;
        conn.transaction(|conn| {
            let created_movie = movies::create(
                conn,
                &NewMovie {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Signal House".to_string(),
                    genre: "Mystery".to_string(),
                    rating: "PG-13".to_string(),
                    duration_minutes: 123,
                    synopsis: "A projectionist discovers that every sold-out screening leaves one impossible extra ticket in the audit log.".to_string(),
                    poster_tone: Some("from-emerald-300 via-zinc-950 to-fuchsia-600".to_string()),
                    is_active: true,
                },
            )?;

            showtimes::create(
                conn,
                &NewShowtime {
                    id: uuid::Uuid::new_v4().to_string(),
                    movie_id: created_movie.id.clone(),
                    starts_at: Utc::now() + Duration::days(2),
                    screen: "Screen 4".to_string(),
                    price_cents: 1299,
                    total_seats: 90,
                },
            )?;

            let movie_rows = vec![created_movie];
            let showtime_rows = showtimes::list_for_movie_id(conn, &movie_rows[0].id)?;
            let showtime_ids: Vec<String> = showtime_rows.iter().map(|showtime| showtime.id.clone()).collect();
            let booking_rows = bookings::list_confirmed_for_showtime_ids(conn, &showtime_ids)?;

            Ok(movie_service::assemble_movies(movie_rows, showtime_rows, booking_rows)
                .into_iter()
                .next()
                .ok_or_else(|| ApiError::not_found("Movie not found after creation."))?)
        })
    })
    .await
    .map_err(ApiError::from)?
}

pub async fn delete_movie(state: AppState, id: String) -> Result<(), ApiError> {
    let pool = state.db.clone();

    tokio::task::spawn_blocking(move || -> Result<(), ApiError> {
        let mut conn = pool.get()?;
        let updated = movies::soft_delete(&mut conn, &id)?;
        if updated == 0 {
            return Err(ApiError::not_found("Movie not found."));
        }

        Ok(())
    })
    .await
    .map_err(ApiError::from)?
}