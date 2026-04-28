use chrono::Utc;

use super::model::AuditEvent;

pub fn build_event(
    action: &str,
    entity_type: &str,
    entity_id: Option<String>,
    actor_user_id: Option<String>,
    tenant_id: Option<String>,
) -> AuditEvent {
    AuditEvent {
        action: action.to_string(),
        entity_type: entity_type.to_string(),
        entity_id,
        actor_user_id,
        tenant_id,
        created_at: Utc::now(),
    }
}
