use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub actor_user_id: Option<String>,
    pub tenant_id: Option<String>,
    pub created_at: DateTime<Utc>,
}
