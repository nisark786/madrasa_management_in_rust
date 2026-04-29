use std::net::SocketAddr;

use axum::{http::StatusCode, response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(index))
        .route("/dashboard/:role", get(index))
        .route("/favicon.ico", get(favicon))
        .route("/health", get(health));

    let port = std::env::var("WEB_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!(%addr, "web server listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind web listener");
    axum::serve(listener, app)
        .await
        .expect("web server failure");
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn health() -> &'static str {
    "ok"
}

async fn favicon() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn index() -> Html<&'static str> {
    Html(include_str!("ui.html"))
}
