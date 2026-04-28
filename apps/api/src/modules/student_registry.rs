mod dto;

use axum::{extract::Query, routing::{get, post}, Json, Router};
use db::repositories::{create_audit_log, create_student_profile, list_student_profiles};
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;
use dto::{CreateStudentRequest, ListStudentsQuery, ListStudentsResponse, StudentResponse};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "student_registry_ok" }))
        .route("", get(list_students))
        .route("", post(create_student))
}

async fn create_student(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateStudentRequest>,
) -> Result<Json<StudentResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let student = create_student_profile(
        &state.pg_pool,
        auth.tenant_id,
        &payload.student_code,
        &payload.full_name,
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("student_profiles_tenant_id_student_code_key") {
            AppError::Conflict("student_code already exists in tenant")
        } else {
            AppError::Db
        }
    })?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "student.create",
        "student_profile",
        Some(student.id),
    )
    .await;

    Ok(Json(StudentResponse {
        id: student.id,
        tenant_id: student.tenant_id,
        full_name: student.full_name,
        student_code: student.student_code,
        status: student.status,
        version: student.version,
    }))
}

async fn list_students(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Query(query): Query<ListStudentsQuery>,
) -> Result<Json<ListStudentsResponse>, AppError> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = list_student_profiles(&state.pg_pool, auth.tenant_id, limit + 1, query.cursor)
        .await
        .map_err(|_| AppError::Db)?;

    let has_more = rows.len() as i64 > limit;
    let mut items = rows
        .into_iter()
        .take(limit as usize)
        .map(|s| StudentResponse {
            id: s.id,
            tenant_id: s.tenant_id,
            full_name: s.full_name,
            student_code: s.student_code,
            status: s.status,
            version: s.version,
        })
        .collect::<Vec<_>>();

    let next_cursor = if has_more {
        items.last().map(|s| s.id)
    } else {
        None
    };

    Ok(Json(ListStudentsResponse {
        items,
        next_cursor,
        has_more,
    }))
}
