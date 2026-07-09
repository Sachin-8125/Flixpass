use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<ConnectionManager<PgConnection>>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }
}