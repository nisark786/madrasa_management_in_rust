use chrono::{DateTime, Utc};
use shared::auth::Role;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct StudentProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub student_code: String,
    pub full_name: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct StaffProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub employee_code: String,
    pub full_name: String,
    pub designation: Option<String>,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub actor_user_id: Option<Uuid>,
    pub actor_role: Option<Role>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Class {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub course_id: Uuid,
    pub teacher_user_id: Option<Uuid>,
    pub class_name: String,
    pub capacity: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AttendanceRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub class_id: Uuid,
    pub student_profile_id: Uuid,
    pub attendance_date: chrono::NaiveDate,
    pub status: String,
    pub notes: Option<String>,
    pub marked_by_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct HifzProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub status: String,
    pub total_chapters_memorized: i32,
    pub total_verses_memorized: i32,
    pub current_juz: Option<i32>,
    pub current_surah: Option<i32>,
    pub current_verse: Option<i32>,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct QuranSession {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub teacher_user_id: Option<Uuid>,
    pub hifz_profile_id: Uuid,
    pub session_date: chrono::NaiveDate,
    pub session_type: String,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TajweedScore {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub quran_session_id: Uuid,
    pub student_profile_id: Uuid,
    pub teacher_user_id: Option<Uuid>,
    pub category: String,
    pub score: i32,
    pub feedback: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct FeePlan {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct StudentEnrollmentFinance {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub fee_plan_id: Uuid,
    pub status: String,
    pub enrollment_date: chrono::NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub invoice_number: String,
    pub amount_cents: i64,
    pub status: String,
    pub due_date: chrono::NaiveDate,
    pub issued_date: chrono::NaiveDate,
    pub notes: Option<String>,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub invoice_id: Uuid,
    pub student_profile_id: Uuid,
    pub amount_cents: i64,
    pub payment_method: String,
    pub payment_date: chrono::NaiveDate,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
