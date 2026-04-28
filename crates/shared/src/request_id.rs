use axum::http::HeaderValue;
use std::error::Error;
use tower_http::request_id::MakeRequestId;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id(&mut self, _request: &axum::http::Request<axum::body::Body>) -> Option<HeaderValue> {
        HeaderValue::from_str(&Uuid::new_v4().to_string()).ok()
    }
}

pub type RequestIdError = Box<dyn Error + Send + Sync>;
