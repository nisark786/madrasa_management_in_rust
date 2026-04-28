mod dto;

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use db::repositories::{
    create_audit_log, create_hifz_profile as db_create_hifz_profile, create_quran_session, create_tajweed_score, get_hifz_profile,
    list_quran_sessions, list_tajweed_scores,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;
use dto::{
    CreateHifzProfileRequest, CreateQuranSessionRequest, CreateTajweedScoreRequest,
    HifzProfileResponse, ListQuranSessionsQuery, ListQuranSessionsResponse, QuranSessionResponse,
    TajweedScoreResponse,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(|| async { "quran_module_ok" }))
        .route("/hifz", post(create_hifz_profile))
        .route("/hifz/:student_id", get(get_hifz_profile_handler))
        .route("/sessions", post(create_session))
        .route("/sessions", get(list_sessions))
        .route("/tajweed", post(create_tajweed_score_handler))
        .route("/tajweed/:session_id", get(list_tajweed_scores_handler))
}

async fn create_hifz_profile(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateHifzProfileRequest>,
) -> Result<Json<HifzProfileResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let hifz = db_create_hifz_profile(&state.pg_pool, auth.tenant_id, payload.student_id)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("hifz_profiles_tenant_id_student_profile_id_key") {
                AppError::Conflict("hifz profile already exists for student")
            } else {
                AppError::Db
            }
        })?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "quran.hifz_profile.create",
        "hifz_profile",
        Some(hifz.id),
    )
    .await;

    Ok(Json(HifzProfileResponse {
        id: hifz.id,
        tenant_id: hifz.tenant_id,
        student_profile_id: hifz.student_profile_id,
        status: hifz.status,
        total_chapters_memorized: hifz.total_chapters_memorized,
        total_verses_memorized: hifz.total_verses_memorized,
        current_juz: hifz.current_juz,
        current_surah: hifz.current_surah,
        current_verse: hifz.current_verse,
    }))
}

async fn get_hifz_profile_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(student_id): Path<Uuid>,
) -> Result<Json<HifzProfileResponse>, AppError> {
    let hifz = get_hifz_profile(&state.pg_pool, auth.tenant_id, student_id)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    Ok(Json(HifzProfileResponse {
        id: hifz.id,
        tenant_id: hifz.tenant_id,
        student_profile_id: hifz.student_profile_id,
        status: hifz.status,
        total_chapters_memorized: hifz.total_chapters_memorized,
        total_verses_memorized: hifz.total_verses_memorized,
        current_juz: hifz.current_juz,
        current_surah: hifz.current_surah,
        current_verse: hifz.current_verse,
    }))
}

async fn create_session(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateQuranSessionRequest>,
) -> Result<Json<QuranSessionResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Get hifz profile
    let hifz = get_hifz_profile(&state.pg_pool, auth.tenant_id, payload.student_id)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    let session = create_quran_session(
        &state.pg_pool,
        auth.tenant_id,
        payload.student_id,
        hifz.id,
        Some(auth.user_id),
        payload.session_date,
        &payload.session_type,
        payload.duration_minutes,
        payload.notes.as_deref(),
    )
    .await
    .map_err(|_| AppError::Db)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "quran.session.create",
        "quran_session",
        Some(session.id),
    )
    .await;

    Ok(Json(QuranSessionResponse {
        id: session.id,
        tenant_id: session.tenant_id,
        student_profile_id: session.student_profile_id,
        teacher_user_id: session.teacher_user_id,
        session_date: session.session_date,
        session_type: session.session_type,
        duration_minutes: session.duration_minutes,
        notes: session.notes,
    }))
}

async fn list_sessions(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Query(query): Query<ListQuranSessionsQuery>,
) -> Result<Json<ListQuranSessionsResponse>, AppError> {
    query
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let limit = 50i64;
    let sessions = list_quran_sessions(&state.pg_pool, auth.tenant_id, query.student_id, limit)
        .await
        .map_err(|_| AppError::Db)?;

    let items = sessions
        .into_iter()
        .map(|s| QuranSessionResponse {
            id: s.id,
            tenant_id: s.tenant_id,
            student_profile_id: s.student_profile_id,
            teacher_user_id: s.teacher_user_id,
            session_date: s.session_date,
            session_type: s.session_type,
            duration_minutes: s.duration_minutes,
            notes: s.notes,
        })
        .collect();

    Ok(Json(ListQuranSessionsResponse { items }))
}

async fn create_tajweed_score_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateTajweedScoreRequest>,
) -> Result<Json<TajweedScoreResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let score = create_tajweed_score(
        &state.pg_pool,
        auth.tenant_id,
        payload.session_id,
        payload.student_id,
        Some(auth.user_id),
        &payload.category,
        payload.score,
        payload.feedback.as_deref(),
    )
    .await
    .map_err(|_| AppError::Db)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "quran.tajweed_score.create",
        "tajweed_score",
        Some(score.id),
    )
    .await;

    Ok(Json(TajweedScoreResponse {
        id: score.id,
        quran_session_id: score.quran_session_id,
        student_profile_id: score.student_profile_id,
        category: score.category,
        score: score.score,
        feedback: score.feedback,
    }))
}

async fn list_tajweed_scores_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Vec<TajweedScoreResponse>>, AppError> {
    let scores = list_tajweed_scores(&state.pg_pool, auth.tenant_id, session_id)
        .await
        .map_err(|_| AppError::Db)?;

    let items = scores
        .into_iter()
        .map(|s| TajweedScoreResponse {
            id: s.id,
            quran_session_id: s.quran_session_id,
            student_profile_id: s.student_profile_id,
            category: s.category,
            score: s.score,
            feedback: s.feedback,
        })
        .collect();

    Ok(Json(items))
}
