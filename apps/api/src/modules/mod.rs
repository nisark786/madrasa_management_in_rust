use axum::Router;
use std::sync::Arc;
use crate::app::AppState;

pub mod academics;
pub mod audit_compliance;
pub mod communication;
pub mod finance;
pub mod identity_access;
pub mod platform_ops;
pub mod quran_module;
pub mod reports_analytics;
pub mod staff_registry;
pub mod student_registry;
pub mod tenant_admin;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/identity", identity_access::router())
        .nest("/tenants", tenant_admin::router())
        .nest("/students", student_registry::router())
        .nest("/staff", staff_registry::router())
        .nest("/academics", academics::router())
        .nest("/quran", quran_module::router())
        .nest("/finance", finance::router())
        .nest("/communication", communication::router())
        .nest("/reports", reports_analytics::router())
        .nest("/audit", audit_compliance::router())
        .nest("/platform", platform_ops::router())
}
