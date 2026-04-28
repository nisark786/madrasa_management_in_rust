use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().to_string();
        let id = format!("req-{}", uuid);
        Some(RequestId::new(
            axum::http::HeaderValue::from_str(&id)
                .unwrap_or_else(|_| axum::http::HeaderValue::from_static("req-unknown"))
        ))
    }
}
