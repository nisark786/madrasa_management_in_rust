mod app;
mod auth;
mod config;
mod error;
mod modules;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use redis::Client as RedisClient;
use sqlx::postgres::PgPoolOptions;
use tower_http::{compression::CompressionLayer, request_id::SetRequestIdLayer, trace::TraceLayer};
use tracing::info;

use crate::app::AppState;
use crate::config::Config;

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();

    let pg_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .expect("failed to connect postgres");

    sqlx::migrate_from!("../../../migrations")
        .run(&pg_pool)
        .await
        .expect("failed to run migrations");

    let redis = RedisClient::open(config.redis_url.clone()).expect("failed to create redis client");

    let state = AppState::new(config.clone(), pg_pool, redis);

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .nest("/api/v1", modules::router())
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::x_request_id(shared::request_id::MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!(%addr, "api server listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("server failure");
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn health() -> &'static str {
    "ok"
}

async fn ready() -> &'static str {
    "ready"
}
