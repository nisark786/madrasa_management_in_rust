use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateHifzProfileRequest {
    pub student_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct HifzProfileResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub status: String,
    pub total_chapters_memorized: i32,
    pub total_verses_memorized: i32,
    pub current_juz: Option<i32>,
    pub current_surah: Option<i32>,
    pub current_verse: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuranSessionRequest {
    pub student_id: Uuid,
    #[validate(length(max = 200))]
    pub notes: Option<String>,
    pub session_date: chrono::NaiveDate,
    pub session_type: String, // sabaq, sabqi, manzil, review
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct QuranSessionResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub teacher_user_id: Option<Uuid>,
    pub session_date: chrono::NaiveDate,
    pub session_type: String,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTajweedScoreRequest {
    pub session_id: Uuid,
    pub student_id: Uuid,
    pub category: String, // makhraj, prolongation, emphasis, stops, general
    #[validate(range(min = 1, max = 10))]
    pub score: i32,
    #[validate(length(max = 500))]
    pub feedback: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TajweedScoreResponse {
    pub id: Uuid,
    pub quran_session_id: Uuid,
    pub student_profile_id: Uuid,
    pub category: String,
    pub score: i32,
    pub feedback: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListQuranSessionsQuery {
    pub student_id: Uuid,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListQuranSessionsResponse {
    pub items: Vec<QuranSessionResponse>,
}
