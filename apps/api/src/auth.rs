use std::sync::Arc;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use shared::auth::Role;
use uuid::Uuid;

use crate::{app::AppState, error::AppError};
use db::repositories::get_user_by_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub tenant_id: String,
    pub role: String,
    pub token_type: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: Role,
}

impl AuthContext {
    pub fn require_manager_or_admin(&self) -> Result<(), AppError> {
        match self.role {
            Role::Manager | Role::PlatformAdmin => Ok(()),
            _ => Err(AppError::Forbidden),
        }
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthContext {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "missing authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "invalid authorization scheme"))?;

        let claims = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(state.config.jwt_access_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid token"))?
        .claims;

        if claims.token_type != "access" {
            return Err((StatusCode::UNAUTHORIZED, "invalid token type"));
        }

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid subject"))?;
        let tenant_id = Uuid::parse_str(&claims.tenant_id)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid tenant"))?;

        let role = match claims.role.as_str() {
            "student" => Role::Student,
            "staff" => Role::Staff,
            "manager" => Role::Manager,
            "platform_admin" => Role::PlatformAdmin,
            _ => return Err((StatusCode::UNAUTHORIZED, "invalid role")),
        };

        let user = get_user_by_id(&state.pg_pool, tenant_id, user_id)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "user lookup failed"))?
            .ok_or((StatusCode::UNAUTHORIZED, "user not found"))?;

        if !user.is_active || user.role != role {
            return Err((StatusCode::UNAUTHORIZED, "invalid user state"));
        }

        Ok(Self {
            user_id,
            tenant_id,
            role,
        })
    }
}
