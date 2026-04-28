use crate::config::Config;
use redis::Client as RedisClient;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pg_pool: PgPool,
    pub redis: RedisClient,
}

impl AppState {
    pub fn new(config: Config, pg_pool: PgPool, redis: RedisClient) -> Arc<Self> {
        Arc::new(Self {
            config,
            pg_pool,
            redis,
        })
    }
}
