mod admin;
mod app;
mod auth;
mod bookings;
mod config;
mod db;
mod error;
mod movies;
mod repositories;
mod state;
mod types;

use std::net::SocketAddr;

use config::Config;
use state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let db_pool = db::pool::build_pool(&config.database_url)?;
    let state = AppState::new(db_pool, config.jwt_secret.clone());
    let app = app::build_app(state, &config.frontend_origin)?;

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    tracing::info!("backend listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}