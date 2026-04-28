use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStaffRequest {
    #[validate(length(min = 3, max = 128, message = "full_name must be between 3 and 128 characters"))]
    pub full_name: String,
    #[validate(length(min = 2, max = 32, message = "employee_code must be between 2 and 32 characters"))]
    pub employee_code: String,
    #[validate(length(max = 128))]
    pub designation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StaffResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub employee_code: String,
    pub full_name: String,
    pub designation: Option<String>,
    pub status: String,
    pub version: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListStaffQuery {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListStaffResponse {
    pub items: Vec<StaffResponse>,
    pub next_cursor: Option<Uuid>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStaffRequest {
    #[validate(length(min = 3, max = 128))]
    pub full_name: Option<String>,
    #[validate(length(max = 128))]
    pub designation: Option<String>,
    #[validate(regex = "STATUS_REGEX")]
    pub status: Option<String>,
}
