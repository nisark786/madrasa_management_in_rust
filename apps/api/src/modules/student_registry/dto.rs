use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStudentRequest {
    #[validate(length(min = 3, max = 128, message = "full_name must be between 3 and 128 characters"))]
    pub full_name: String,
    #[validate(length(min = 2, max = 32, message = "student_code must be between 2 and 32 characters"))]
    pub student_code: String,
}

#[derive(Debug, Serialize)]
pub struct StudentResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub full_name: String,
    pub student_code: String,
    pub status: String,
    pub version: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListStudentsQuery {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListStudentsResponse {
    pub items: Vec<StudentResponse>,
    pub next_cursor: Option<Uuid>,
    pub has_more: bool,
}
