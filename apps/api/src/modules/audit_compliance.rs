mod model;
mod service;

use axum::{routing::get, Json, Router};
use db::repositories::list_audit_logs;
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;
use model::AuditEvent;
use service::build_event;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(|| async { "audit_compliance_ok" }))
        .route("/sample", get(sample_event))
        .route("/", get(list_events))
}

async fn sample_event(auth: AuthContext) -> Result<Json<AuditEvent>, AppError> {
    let event = build_event(
        "audit.sample.read",
        "audit_event",
        None,
        Some(auth.user_id.to_string()),
        Some(auth.tenant_id.to_string()),
    );
    Ok(Json(event))
}

async fn list_events(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
) -> Result<Json<Vec<AuditEvent>>, AppError> {
    let rows = list_audit_logs(&state.pg_pool, auth.tenant_id, 50)
        .await
        .map_err(|_| AppError::Db)?;

    let events = rows
        .into_iter()
        .map(|r| AuditEvent {
            action: r.action,
            entity_type: r.entity_type,
            entity_id: r.entity_id.map(|v| v.to_string()),
            actor_user_id: r.actor_user_id.map(|v| v.to_string()),
            tenant_id: Some(r.tenant_id.to_string()),
            created_at: r.created_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(events))
}
