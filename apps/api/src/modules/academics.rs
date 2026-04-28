use axum::{extract::State, routing::{get, post}, Json, Router};
use chrono::NaiveDate;
use db::repositories::{create_audit_log, create_class, create_course, list_courses, mark_attendance};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(|| async { "academics_ok" }))
        .route("/courses", post(create_course_handler).get(list_courses_handler))
        .route("/classes", post(create_class_handler))
        .route("/attendance", post(mark_attendance_handler))
}

#[derive(Debug, Deserialize, Validate)]
struct CreateCourseRequest {
    #[validate(length(min = 2, max = 32, message = "course code must be between 2 and 32 characters"))]
    code: String,
    #[validate(length(min = 2, max = 128, message = "course name must be between 2 and 128 characters"))]
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct CourseResponse {
    id: Uuid,
    tenant_id: Uuid,
    code: String,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
struct CreateClassRequest {
    course_id: Uuid,
    #[validate(length(min = 2, max = 128, message = "class name must be between 2 and 128 characters"))]
    class_name: String,
    teacher_user_id: Option<Uuid>,
    capacity: Option<i32>,
}

#[derive(Debug, Serialize)]
struct ClassResponse {
    id: Uuid,
    tenant_id: Uuid,
    course_id: Uuid,
    class_name: String,
    capacity: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct MarkAttendanceRequest {
    class_id: Uuid,
    student_profile_id: Uuid,
    attendance_date: String,
    status: String,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
struct AttendanceResponse {
    id: Uuid,
    tenant_id: Uuid,
    class_id: Uuid,
    student_profile_id: Uuid,
    attendance_date: String,
    status: String,
}

async fn create_course_handler(
    State(state): State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateCourseRequest>,
) -> Result<Json<CourseResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let course = create_course(
        &state.pg_pool,
        auth.tenant_id,
        &payload.code,
        &payload.name,
        payload.description.as_deref(),
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("courses_tenant_id_code_key") {
            AppError::Conflict("course code already exists in tenant")
        } else {
            AppError::Db
        }
    })?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "academics.course.create",
        "course",
        Some(course.id),
    )
    .await;

    Ok(Json(CourseResponse {
        id: course.id,
        tenant_id: course.tenant_id,
        code: course.code,
        name: course.name,
        description: course.description,
    }))
}

async fn list_courses_handler(
    State(state): State<std::sync::Arc<AppState>>,
    auth: AuthContext,
) -> Result<Json<Vec<CourseResponse>>, AppError> {
    let courses = list_courses(&state.pg_pool, auth.tenant_id, 100)
        .await
        .map_err(|_| AppError::Db)?;

    Ok(Json(
        courses
            .into_iter()
            .map(|c| CourseResponse {
                id: c.id,
                tenant_id: c.tenant_id,
                code: c.code,
                name: c.name,
                description: c.description,
            })
            .collect(),
    ))
}

async fn create_class_handler(
    State(state): State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateClassRequest>,
) -> Result<Json<ClassResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let capacity = payload.capacity.unwrap_or(40).max(1);

    let class_row = create_class(
        &state.pg_pool,
        auth.tenant_id,
        payload.course_id,
        &payload.class_name,
        payload.teacher_user_id,
        capacity,
    )
    .await
    .map_err(|_| AppError::Db)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "academics.class.create",
        "class",
        Some(class_row.id),
    )
    .await;

    Ok(Json(ClassResponse {
        id: class_row.id,
        tenant_id: class_row.tenant_id,
        course_id: class_row.course_id,
        class_name: class_row.class_name,
        capacity: class_row.capacity,
    }))
}

async fn mark_attendance_handler(
    State(state): State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<MarkAttendanceRequest>,
) -> Result<Json<AttendanceResponse>, AppError> {
    match auth.role {
        shared::auth::Role::Staff | shared::auth::Role::Manager | shared::auth::Role::PlatformAdmin => {}
        _ => return Err(AppError::Forbidden),
    }

    let attendance_date = NaiveDate::parse_from_str(&payload.attendance_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("attendance_date must be YYYY-MM-DD".to_string()))?;

    let status = payload.status.to_lowercase();
    if !matches!(status.as_str(), "present" | "absent" | "late" | "excused") {
        return Err(AppError::Validation(
            "status must be present|absent|late|excused".to_string(),
        ));
    }

    let row = mark_attendance(
        &state.pg_pool,
        auth.tenant_id,
        payload.class_id,
        payload.student_profile_id,
        attendance_date,
        &status,
        payload.notes.as_deref(),
        Some(auth.user_id),
    )
    .await
    .map_err(|_| AppError::Db)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role),
        "academics.attendance.mark",
        "attendance_record",
        Some(row.id),
    )
    .await;

    Ok(Json(AttendanceResponse {
        id: row.id,
        tenant_id: row.tenant_id,
        class_id: row.class_id,
        student_profile_id: row.student_profile_id,
        attendance_date: row.attendance_date.to_string(),
        status: row.status,
    }))
}
