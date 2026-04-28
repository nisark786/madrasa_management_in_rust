use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::auth::Role;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_ttl_min: i64,
    pub refresh_ttl_days: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub tenant_id: String,
    pub role: String,
    pub token_type: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

pub struct IssuedTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_in: i64,
    pub refresh_jti: String,
    pub refresh_expires_at: i64,
}

pub fn hash_password(plain: &str) -> Result<String, argon2::password_hash::Error> {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(plain.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn verify_password(hash: &str, plain: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let parsed = match PasswordHash::new(hash) {
        Ok(v) => v,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
}

pub fn issue_tokens(
    user_id: Uuid,
    tenant_id: Uuid,
    role: Role,
    cfg: &TokenConfig,
) -> Result<IssuedTokens, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let access_exp = now + Duration::minutes(cfg.access_ttl_min);
    let refresh_exp = now + Duration::days(cfg.refresh_ttl_days);

    let role = role.as_str().to_string();

    let access_claims = Claims {
        sub: user_id.to_string(),
        tenant_id: tenant_id.to_string(),
        role: role.clone(),
        token_type: "access".to_string(),
        exp: access_exp.timestamp(),
        iat: now.timestamp(),
        jti: Uuid::new_v4().to_string(),
    };

    let refresh_jti = Uuid::new_v4().to_string();
    let refresh_claims = Claims {
        sub: user_id.to_string(),
        tenant_id: tenant_id.to_string(),
        role,
        token_type: "refresh".to_string(),
        exp: refresh_exp.timestamp(),
        iat: now.timestamp(),
        jti: refresh_jti.clone(),
    };

    let header = Header::new(Algorithm::HS256);
    let access_token = encode(
        &header,
        &access_claims,
        &EncodingKey::from_secret(cfg.access_secret.as_bytes()),
    )?;
    let refresh_token = encode(
        &header,
        &refresh_claims,
        &EncodingKey::from_secret(cfg.refresh_secret.as_bytes()),
    )?;

    Ok(IssuedTokens {
        access_token,
        refresh_token,
        access_expires_in: cfg.access_ttl_min * 60,
        refresh_jti,
        refresh_expires_at: refresh_exp.timestamp(),
    })
}

pub fn decode_refresh_token(
    token: &str,
    cfg: &TokenConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.refresh_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?
    .claims;

    if claims.token_type != "refresh" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(claims)
}
