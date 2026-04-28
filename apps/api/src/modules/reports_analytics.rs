use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/health", get(|| async { "reports_analytics_ok" }))
}
