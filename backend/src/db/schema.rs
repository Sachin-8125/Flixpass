diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    movies (id) {
        id -> Text,
        title -> Text,
        genre -> Text,
        rating -> Text,
        duration_minutes -> Int4,
        synopsis -> Text,
        poster_tone -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    showtimes (id) {
        id -> Text,
        movie_id -> Text,
        starts_at -> Timestamptz,
        screen -> Text,
        price_cents -> Int4,
        total_seats -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    bookings (id) {
        id -> Text,
        user_id -> Text,
        showtime_id -> Text,
        seats -> Array<Int4>,
        total_cents -> Int4,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(showtimes -> movies (movie_id));
diesel::joinable!(bookings -> users (user_id));
diesel::joinable!(bookings -> showtimes (showtime_id));

diesel::allow_tables_to_appear_in_same_query!(users, movies, showtimes, bookings);