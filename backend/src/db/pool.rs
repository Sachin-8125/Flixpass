use anyhow::Context;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn build_pool(database_url: &str) -> anyhow::Result<Pool<ConnectionManager<PgConnection>>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .context("failed to create PostgreSQL connection pool")
}