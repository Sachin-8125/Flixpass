use std::collections::HashMap;

use crate::{
    db::models::{MovieRow, ShowtimeRow},
    error::ApiError,
    movies::dto::{MovieDto, ShowtimeDto},
    repositories::{bookings, movies, showtimes},
    state::AppState,
};

pub async fn list_movies(state: AppState) -> Result<Vec<MovieDto>, ApiError> {
    let pool = state.db.clone();

    tokio::task::spawn_blocking(move || -> Result<Vec<MovieDto>, ApiError> {
        let mut conn = pool.get()?;
        let movie_rows = movies::list_active(&mut conn)?;
        let movie_ids: Vec<String> = movie_rows.iter().map(|movie| movie.id.clone()).collect();
        let showtime_rows = showtimes::list_for_movie_ids(&mut conn, &movie_ids)?;
        let showtime_ids: Vec<String> = showtime_rows
            .iter()
            .map(|showtime| showtime.id.clone())
            .collect();
        let booking_rows = bookings::list_confirmed_for_showtime_ids(&mut conn, &showtime_ids)?;

        Ok(assemble_movies(movie_rows, showtime_rows, booking_rows))
    })
    .await
    .map_err(ApiError::from)?
}

pub fn assemble_movies(
    movie_rows: Vec<MovieRow>,
    showtime_rows: Vec<ShowtimeRow>,
    booking_rows: Vec<crate::db::models::BookingRow>,
) -> Vec<MovieDto> {
    let mut booked_seats_by_showtime: HashMap<String, Vec<i32>> = HashMap::new();
    for booking in booking_rows {
        booked_seats_by_showtime
            .entry(booking.showtime_id)
            .or_default()
            .extend(booking.seats);
    }

    let mut showtimes_by_movie: HashMap<String, Vec<ShowtimeDto>> = HashMap::new();
    for showtime in showtime_rows {
        let booked_seats = booked_seats_by_showtime
            .remove(&showtime.id)
            .unwrap_or_default();
        showtimes_by_movie
            .entry(showtime.movie_id.clone())
            .or_default()
            .push(ShowtimeDto {
                id: showtime.id,
                starts_at: showtime.starts_at,
                screen: showtime.screen,
                price_cents: showtime.price_cents,
                total_seats: showtime.total_seats,
                booked_seats,
            });
    }

    movie_rows
        .into_iter()
        .map(|movie| MovieDto {
            id: movie.id.clone(),
            title: movie.title,
            genre: movie.genre,
            rating: movie.rating,
            duration_minutes: movie.duration_minutes,
            synopsis: movie.synopsis,
            poster_tone: movie.poster_tone,
            showtimes: showtimes_by_movie.remove(&movie.id).unwrap_or_default(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::db::models::BookingRow;

    #[test]
    fn assemble_movies_groups_showtimes_and_booked_seats() {
        let now = Utc::now();
        let movies = vec![
            MovieRow {
                id: "movie-1".to_string(),
                title: "First".to_string(),
                genre: "Drama".to_string(),
                rating: "PG".to_string(),
                duration_minutes: 120,
                synopsis: "A story.".to_string(),
                poster_tone: Some("from-a to-b".to_string()),
                is_active: true,
                created_at: now,
                updated_at: now,
            },
            MovieRow {
                id: "movie-2".to_string(),
                title: "Second".to_string(),
                genre: "Action".to_string(),
                rating: "PG-13".to_string(),
                duration_minutes: 95,
                synopsis: "Another story.".to_string(),
                poster_tone: None,
                is_active: true,
                created_at: now,
                updated_at: now,
            },
        ];
        let showtimes = vec![
            ShowtimeRow {
                id: "show-1".to_string(),
                movie_id: "movie-1".to_string(),
                starts_at: now,
                screen: "Screen 1".to_string(),
                price_cents: 1200,
                total_seats: 80,
                created_at: now,
                updated_at: now,
            },
            ShowtimeRow {
                id: "show-2".to_string(),
                movie_id: "movie-1".to_string(),
                starts_at: now,
                screen: "Screen 2".to_string(),
                price_cents: 1400,
                total_seats: 90,
                created_at: now,
                updated_at: now,
            },
        ];
        let bookings = vec![
            BookingRow {
                id: "booking-1".to_string(),
                user_id: "user-1".to_string(),
                showtime_id: "show-1".to_string(),
                seats: vec![1, 2],
                total_cents: 2400,
                status: "CONFIRMED".to_string(),
                created_at: now,
                updated_at: now,
            },
            BookingRow {
                id: "booking-2".to_string(),
                user_id: "user-2".to_string(),
                showtime_id: "show-1".to_string(),
                seats: vec![5],
                total_cents: 1200,
                status: "CONFIRMED".to_string(),
                created_at: now,
                updated_at: now,
            },
        ];

        let assembled = assemble_movies(movies, showtimes, bookings);

        assert_eq!(assembled.len(), 2);
        assert_eq!(assembled[0].showtimes.len(), 2);
        assert_eq!(assembled[0].showtimes[0].booked_seats, vec![1, 2, 5]);
        assert_eq!(assembled[1].showtimes.len(), 0);
    }
}