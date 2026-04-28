mod dto;
mod service;

use axum::{extract::State, routing::{get, post}, Json, Router};
use db::repositories::{create_audit_log, get_tenant_by_slug, get_user_by_email};
use redis::AsyncCommands;
use shared::auth::Role;
use uuid::Uuid;
use validator::Validate;

use crate::{app::AppState, error::AppError};
use dto::{BootstrapRequest, LoginRequest, RefreshRequest, RegisterRequest, TokenResponse};
use service::{decode_refresh_token, hash_password, issue_tokens, verify_password, TokenConfig};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "identity_access_ok" }))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh))
        .route("/bootstrap", post(bootstrap))
}

async fn login(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let tenant = get_tenant_by_slug(&state.pg_pool, &payload.tenant_slug)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::Unauthorized)?;

    if !tenant.is_active {
        return Err(AppError::Forbidden);
    }

    if payload.role == "platform_admin" {
        return Err(AppError::Forbidden);
    }

    let user = get_user_by_email(&state.pg_pool, tenant.id, &payload.email)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::Unauthorized)?;

    if !user.is_active || !verify_password(&user.password_hash, &payload.password) {
        return Err(AppError::Unauthorized);
    }

    let cfg = TokenConfig {
        access_secret: state.config.jwt_access_secret.clone(),
        refresh_secret: state.config.jwt_refresh_secret.clone(),
        access_ttl_min: state.config.jwt_access_ttl_min,
        refresh_ttl_days: state.config.jwt_refresh_ttl_days,
    };

    let issued = issue_tokens(user.id, tenant.id, user.role, &cfg).map_err(|_| AppError::Internal)?;

    let mut redis = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;
    let key = format!("refresh:{}:{}", user.id, issued.refresh_jti);
    let ttl = (issued.refresh_expires_at - chrono::Utc::now().timestamp()).max(1) as u64;
    let _: () = redis
        .set_ex(key, "valid", ttl)
        .await
        .map_err(|_| AppError::Internal)?;

    let _ = create_audit_log(
        &state.pg_pool,
        tenant.id,
        Some(user.id),
        Some(user.role),
        "auth.login",
        "user",
        Some(user.id),
    )
    .await;

    Ok(Json(TokenResponse {
        access_token: issued.access_token,
        refresh_token: issued.refresh_token,
        token_type: "Bearer",
        expires_in: issued.access_expires_in,
        user_id: user.id,
        tenant_id: tenant.id,
        role: user.role.as_str().to_string(),
        email: user.email.clone(),
    }))
}

async fn register(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let tenant = get_tenant_by_slug(&state.pg_pool, &payload.tenant_slug)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    if !tenant.is_active {
        return Err(AppError::Forbidden);
    }

    let role = match payload.role.as_str() {
        "student" => Role::Student,
        "staff" => Role::Staff,
        "manager" => Role::Manager,
        "platform_admin" => Role::PlatformAdmin,
        _ => return Err(AppError::Validation("invalid role".to_string())),
    };

    let existing = get_user_by_email(&state.pg_pool, tenant.id, &payload.email)
        .await
        .map_err(|_| AppError::Db)?;
    if existing.is_some() {
        return Err(AppError::Conflict("email already exists in tenant"));
    }

    let mut redis = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;

    let password_hash = hash_password(&payload.password).map_err(|_| AppError::Internal)?;

    let user = sqlx::query_as::<_, db::models::User>(
        r#"
        insert into users (tenant_id, email, password_hash, role)
        values ($1, $2, $3, $4)
        returning id, tenant_id, email, password_hash, role, is_active, created_at, updated_at
        "#,
    )
    .bind(tenant.id)
    .bind(payload.email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|_| AppError::Db)?;

    let cfg = TokenConfig {
        access_secret: state.config.jwt_access_secret.clone(),
        refresh_secret: state.config.jwt_refresh_secret.clone(),
        access_ttl_min: state.config.jwt_access_ttl_min,
        refresh_ttl_days: state.config.jwt_refresh_ttl_days,
    };

    let issued = issue_tokens(user.id, tenant.id, user.role, &cfg).map_err(|_| AppError::Internal)?;

    let key = format!("refresh:{}:{}", user.id, issued.refresh_jti);
    let ttl = (issued.refresh_expires_at - chrono::Utc::now().timestamp()).max(1) as u64;
    let _: () = redis
        .set_ex(key, "valid", ttl)
        .await
        .map_err(|_| AppError::Internal)?;

    let _ = create_audit_log(
        &state.pg_pool,
        tenant.id,
        Some(user.id),
        Some(user.role),
        "auth.register",
        "user",
        Some(user.id),
    )
     .await;

    Ok(Json(TokenResponse {
        access_token: issued.access_token,
        refresh_token: issued.refresh_token,
        token_type: "Bearer",
        expires_in: issued.access_expires_in,
        user_id: user.id,
        tenant_id: tenant.id,
        role: user.role.as_str().to_string(),
        email: user.email.clone(),
    }))
}

async fn refresh(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let cfg = TokenConfig {
        access_secret: state.config.jwt_access_secret.clone(),
        refresh_secret: state.config.jwt_refresh_secret.clone(),
        access_ttl_min: state.config.jwt_access_ttl_min,
        refresh_ttl_days: state.config.jwt_refresh_ttl_days,
    };

    let claims = decode_refresh_token(&payload.refresh_token, &cfg).map_err(|_| AppError::Unauthorized)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    let tenant_id = Uuid::parse_str(&claims.tenant_id).map_err(|_| AppError::Unauthorized)?;

    let role = match claims.role.as_str() {
        "student" => Role::Student,
        "staff" => Role::Staff,
        "manager" => Role::Manager,
        "platform_admin" => Role::PlatformAdmin,
        _ => return Err(AppError::Unauthorized),
    };

    let mut redis = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;
    let old_key = format!("refresh:{}:{}", user_id, claims.jti);
    let current: Option<String> = redis.get(&old_key).await.map_err(|_| AppError::Internal)?;
    if current.as_deref() != Some("valid") {
        return Err(AppError::Unauthorized);
    }

     let _: () = redis.del(old_key).await.map_err(|_| AppError::Internal)?;

    let issued = issue_tokens(user_id, tenant_id, role.clone(), &cfg).map_err(|_| AppError::Internal)?;
    
    // Fetch user to get email
    let user = sqlx::query_as::<_, db::models::User>(
        "select id, tenant_id, email, password_hash, role, is_active, created_at, updated_at from users where id = $1 and tenant_id = $2"
    )
    .bind(user_id)
    .bind(tenant_id)
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|_| AppError::Unauthorized)?;
    
    let new_key = format!("refresh:{}:{}", user_id, issued.refresh_jti);
    let ttl = (issued.refresh_expires_at - chrono::Utc::now().timestamp()).max(1) as u64;
    let _: () = redis
        .set_ex(new_key, "valid", ttl)
        .await
        .map_err(|_| AppError::Internal)?;

    let _ = create_audit_log(
        &state.pg_pool,
        tenant_id,
        Some(user_id),
        Some(role.clone()),
        "auth.refresh",
        "user",
        Some(user_id),
    )
    .await;

    Ok(Json(TokenResponse {
        access_token: issued.access_token,
        refresh_token: issued.refresh_token,
        token_type: "Bearer",
        expires_in: issued.access_expires_in,
        user_id: user.id,
        tenant_id: tenant_id,
        role: role.as_str().to_string(),
        email: user.email.clone(),
    }))
}

async fn bootstrap(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<BootstrapRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    if !state.config.allow_bootstrap {
        return Err(AppError::Forbidden);
    }

    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let existing_tenant = get_tenant_by_slug(&state.pg_pool, &payload.tenant_slug)
        .await
        .map_err(|_| AppError::Db)?;
    if existing_tenant.is_some() {
        return Err(AppError::Conflict("tenant slug already exists"));
    }

    let tenant = sqlx::query_as::<_, db::models::Tenant>(
        r#"
        insert into tenants (name, slug)
        values ($1, $2)
        returning id, name, slug, is_active, created_at, updated_at
        "#,
    )
    .bind(payload.tenant_name)
    .bind(payload.tenant_slug)
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|_| AppError::Db)?;

    let password_hash = hash_password(&payload.password).map_err(|_| AppError::Internal)?;
    let role = Role::PlatformAdmin;
    let user = sqlx::query_as::<_, db::models::User>(
        r#"
        insert into users (tenant_id, email, password_hash, role)
        values ($1, $2, $3, $4)
        returning id, tenant_id, email, password_hash, role, is_active, created_at, updated_at
        "#,
    )
    .bind(tenant.id)
    .bind(payload.email)
    .bind(password_hash)
    .bind(role.clone())
    .fetch_one(&state.pg_pool)
    .await
    .map_err(|_| AppError::Db)?;

    let cfg = TokenConfig {
        access_secret: state.config.jwt_access_secret.clone(),
        refresh_secret: state.config.jwt_refresh_secret.clone(),
        access_ttl_min: state.config.jwt_access_ttl_min,
        refresh_ttl_days: state.config.jwt_refresh_ttl_days,
    };

    let issued = issue_tokens(user.id, tenant.id, role.clone(), &cfg).map_err(|_| AppError::Internal)?;

    let mut redis = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;
    let key = format!("refresh:{}:{}", user.id, issued.refresh_jti);
    let ttl = (issued.refresh_expires_at - chrono::Utc::now().timestamp()).max(1) as u64;
    let _: () = redis
        .set_ex(key, "valid", ttl)
        .await
        .map_err(|_| AppError::Internal)?;

    let _ = create_audit_log(
        &state.pg_pool,
        tenant.id,
        Some(user.id),
        Some(role),
        "auth.bootstrap",
        "user",
        Some(user.id),
    )
    .await;

    Ok(Json(TokenResponse {
        access_token: issued.access_token,
        refresh_token: issued.refresh_token,
        token_type: "Bearer",
        expires_in: issued.access_expires_in,
    }))
}
