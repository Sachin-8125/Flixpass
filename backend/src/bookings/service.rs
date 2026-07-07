use diesel::Connection;

use crate::{
    auth::middleware::Principal,
    bookings::dto::{BookingDto, CreateBookingRequest},
    db::models::NewBooking,
    error::ApiError,
    repositories::{bookings, movies, showtimes},
    state::AppState,
    types::BookingStatus,
};

pub async fn create_booking(
    state: AppState,
    principal: Principal,
    input: CreateBookingRequest,
) -> Result<BookingDto, ApiError> {
    if input.seats.is_empty() || input.seats.len() > 8 || input.seats.iter().any(|seat| *seat < 1) {
        return Err(ApiError::bad_request("Choose between 1 and 8 valid seats."));
    }

    let pool = state.db.clone();

    tokio::task::spawn_blocking(move || -> Result<BookingDto, ApiError> {
        let mut conn = pool.get()?;
        conn.transaction(|conn| {
            let show = showtimes::find_for_update(conn, &input.showtime_id)?
                .ok_or_else(|| ApiError::not_found("Showtime not found."))?;
            let movie_title = movies::find_by_id(conn, &show.movie_id)?
                .map(|movie| movie.title)
                .unwrap_or_default();

            let mut seats = input.seats.clone();
            seats.sort_unstable();
            seats.dedup();

            if seats.iter().any(|seat| *seat > show.total_seats) {
                return Err(ApiError::bad_request(
                    "One or more seats are outside this screen.",
                ));
            }

            let taken: Vec<i32> = bookings::list_confirmed_for_showtime(conn, &show.id)?
                .into_iter()
                .flat_map(|booking| booking.seats)
                .collect();

            if seats.iter().any(|seat| taken.contains(seat)) {
                return Err(ApiError::conflict("One or more seats are already booked."));
            }

            let booking = bookings::create(
                conn,
                &NewBooking {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id: principal.id.clone(),
                    showtime_id: show.id.clone(),
                    seats: seats.clone(),
                    total_cents: seats.len() as i32 * show.price_cents,
                    status: BookingStatus::Confirmed.as_db().to_string(),
                },
            )?;

            Ok(BookingDto {
                id: booking.id,
                movie_title,
                seats: booking.seats,
                total_cents: booking.total_cents,
                status: BookingStatus::Confirmed,
            })
        })
    })
    .await
    .map_err(ApiError::from)?
}

pub async fn my_bookings(
    state: AppState,
    principal: Principal,
) -> Result<Vec<BookingDto>, ApiError> {
    let pool = state.db.clone();

    tokio::task::spawn_blocking(move || -> Result<Vec<BookingDto>, ApiError> {
        let mut conn = pool.get()?;
        let rows = bookings::list_for_user_with_movie(&mut conn, &principal.id)?;

        rows.into_iter()
            .map(|(id, movie_title, seats, total_cents, status)| {
                let status = BookingStatus::from_db(&status).ok_or_else(|| {
                    ApiError::Internal(anyhow::anyhow!("unknown booking status in database"))
                })?;
                Ok(BookingDto {
                    id,
                    movie_title,
                    seats,
                    total_cents,
                    status,
                })
            })
            .collect()
    })
    .await
    .map_err(ApiError::from)?
}