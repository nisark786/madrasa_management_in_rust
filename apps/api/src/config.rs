#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub app_env: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_ttl_min: i64,
    pub jwt_refresh_ttl_days: i64,
    pub allow_bootstrap: bool,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("APP_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);

        Self {
            port,
            app_env: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/madrasa".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            jwt_access_secret: std::env::var("JWT_ACCESS_SECRET")
                .unwrap_or_else(|_| "dev_access_secret_change_me".to_string()),
            jwt_refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .unwrap_or_else(|_| "dev_refresh_secret_change_me".to_string()),
            jwt_access_ttl_min: std::env::var("JWT_ACCESS_TTL_MIN")
                .ok()
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(15),
            jwt_refresh_ttl_days: std::env::var("JWT_REFRESH_TTL_DAYS")
                .ok()
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(30),
            allow_bootstrap: std::env::var("ALLOW_BOOTSTRAP")
                .ok()
                .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
                .unwrap_or(true),
        }
    }
}
