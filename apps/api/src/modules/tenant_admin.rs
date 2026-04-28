use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "tenant_admin_ok" }))
        .route("", post(create_tenant))
}

#[derive(Debug, Deserialize, Validate)]
struct CreateTenantRequest {
    #[validate(length(min = 2, max = 120, message = "tenant name must be between 2 and 120 characters"))]
    name: String,
    #[validate(length(min = 2, max = 64, message = "tenant slug must be between 2 and 64 characters"))]
    slug: String,
}

#[derive(Debug, Serialize)]
struct TenantResponse {
    id: Uuid,
    name: String,
    slug: String,
    is_active: bool,
}

async fn create_tenant(
    State(state): State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<Json<TenantResponse>, AppError> {
    if auth.role != shared::auth::Role::PlatformAdmin {
        return Err(AppError::Forbidden);
    }

    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let tenant = sqlx::query_as::<_, db::models::Tenant>(
        r#"
        insert into tenants (name, slug)
        values ($1, $2)
        returning id, name, slug, is_active, created_at, updated_at
        "#,
    )
    .bind(payload.name)
    .bind(payload.slug)
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("tenants_slug_key") {
            AppError::Conflict("tenant slug already exists")
        } else {
            AppError::Db
        }
    })?;

    Ok(Json(TenantResponse {
        id: tenant.id,
        name: tenant.name,
        slug: tenant.slug,
        is_active: tenant.is_active,
    }))
}
