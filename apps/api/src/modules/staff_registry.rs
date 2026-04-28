mod dto;

use axum::{
    extract::{Path, Query},
    routing::{get, post, put},
    Json, Router,
};
use db::repositories::{create_audit_log, create_staff_profile, get_staff_profile_by_id, list_staff_profiles, update_staff_profile};
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;
use dto::{CreateStaffRequest, ListStaffQuery, ListStaffResponse, StaffResponse, UpdateStaffRequest};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "staff_registry_ok" }))
        .route("", get(list_staff))
        .route("", post(create_staff))
        .route("/:staff_id", get(get_staff))
        .route("/:staff_id", put(update_staff))
}

async fn create_staff(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateStaffRequest>,
) -> Result<Json<StaffResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let staff = create_staff_profile(
        &state.pg_pool,
        auth.tenant_id,
        &payload.employee_code,
        &payload.full_name,
        payload.designation.as_deref(),
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("staff_profiles_tenant_id_employee_code_key") {
            AppError::Conflict("employee_code already exists in tenant")
        } else {
            AppError::Db
        }
    })?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "staff.create",
        "staff_profile",
        Some(staff.id),
    )
    .await;

    Ok(Json(StaffResponse {
        id: staff.id,
        tenant_id: staff.tenant_id,
        employee_code: staff.employee_code,
        full_name: staff.full_name,
        designation: staff.designation,
        status: staff.status,
        version: staff.version,
    }))
}

async fn list_staff(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Query(query): Query<ListStaffQuery>,
) -> Result<Json<ListStaffResponse>, AppError> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = list_staff_profiles(&state.pg_pool, auth.tenant_id, limit + 1, query.cursor)
        .await
        .map_err(|_| AppError::Db)?;

    let has_more = rows.len() as i64 > limit;
    let mut items = rows
        .into_iter()
        .take(limit as usize)
        .map(|s| StaffResponse {
            id: s.id,
            tenant_id: s.tenant_id,
            employee_code: s.employee_code,
            full_name: s.full_name,
            designation: s.designation,
            status: s.status,
            version: s.version,
        })
        .collect::<Vec<_>>();

    let next_cursor = if has_more {
        items.last().map(|s| s.id)
    } else {
        None
    };

    Ok(Json(ListStaffResponse {
        items,
        next_cursor,
        has_more,
    }))
}

async fn get_staff(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(staff_id): Path<Uuid>,
) -> Result<Json<StaffResponse>, AppError> {
    let staff = get_staff_profile_by_id(&state.pg_pool, auth.tenant_id, staff_id)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    Ok(Json(StaffResponse {
        id: staff.id,
        tenant_id: staff.tenant_id,
        employee_code: staff.employee_code,
        full_name: staff.full_name,
        designation: staff.designation,
        status: staff.status,
        version: staff.version,
    }))
}

async fn update_staff(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(staff_id): Path<Uuid>,
    Json(payload): Json<UpdateStaffRequest>,
) -> Result<Json<StaffResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let staff = update_staff_profile(
        &state.pg_pool,
        auth.tenant_id,
        staff_id,
        payload.full_name.as_deref(),
        payload.designation.as_deref(),
        payload.status.as_deref(),
    )
    .await
    .map_err(|_| AppError::Db)?
    .ok_or(AppError::NotFound)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "staff.update",
        "staff_profile",
        Some(staff.id),
    )
    .await;

    Ok(Json(StaffResponse {
        id: staff.id,
        tenant_id: staff.tenant_id,
        employee_code: staff.employee_code,
        full_name: staff.full_name,
        designation: staff.designation,
        status: staff.status,
        version: staff.version,
    }))
}
