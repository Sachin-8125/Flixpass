use std::env;

use anyhow::Context;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub frontend_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            jwt_secret: env::var("JWT_SECRET").context("JWT_SECRET is required")?,
            frontend_origin: env::var("FRONTEND_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        })
    }
}