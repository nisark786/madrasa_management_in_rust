use axum::{routing::get, Router};
use std::sync::Arc;
use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(|| async { "platform_ops_ok" }))
}
