use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email is invalid"))]
    pub email: String,
    #[validate(length(min = 6, message = "password must be at least 6 characters"))]
    pub password: String,
    pub tenant_slug: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 10, message = "refresh_token is invalid"))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 2, message = "tenant slug is required"))]
    pub tenant_slug: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BootstrapRequest {
    #[validate(length(min = 2, max = 120, message = "tenant name must be between 2 and 120 characters"))]
    pub tenant_name: String,
    #[validate(length(min = 2, max = 64, message = "tenant slug must be between 2 and 64 characters"))]
    pub tenant_slug: String,
    #[validate(email(message = "email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}
