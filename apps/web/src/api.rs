use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub full_name: String,
    pub student_code: String,
    pub status: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub employee_code: String,
    pub full_name: String,
    pub designation: Option<String>,
    pub status: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub student_profile_id: Uuid,
    pub invoice_number: String,
    pub amount_cents: i64,
    pub status: String,
    pub due_date: chrono::NaiveDate,
    pub issued_date: chrono::NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeePlanDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
}

pub async fn api_call<T: for<'de> Deserialize<'de>>(
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Option<String>,
) -> Result<T, String> {
    use gloo_utils::format::JsValueSerdeExt;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Headers};

    let url = format!("/api/v1{}", path);
    let mut init = RequestInit::new();
    init.method(method);

    let headers = Headers::new();
    if let Some(token) = token {
        headers
            .set("Authorization", &format!("Bearer {}", token))
            .map_err(|_| "Failed to set auth header")?;
    }
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| "Failed to set content-type")?;

    init.headers(&headers);

    if let Some(body) = body {
        init.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    }

    let request = Request::new_with_str_and_init(&url, &init)
        .map_err(|_| "Failed to create request")?;

    let window = web_sys::window().ok_or("No window object")?;
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Fetch failed")?;

    let response: web_sys::Response = response
        .dyn_into()
        .map_err(|_| "Response conversion failed")?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    let body = JsFuture::from(
        response
            .text()
            .map_err(|_| "Text extraction failed")?,
    )
    .await
    .map_err(|_| "Body extraction failed")?;

    let response_text = body
        .as_string()
        .ok_or("Response not text")?;

    let parsed: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|_| "JSON parse failed")?;

    if let Some(data) = parsed.get("data") {
        serde_json::from_value(data.clone()).map_err(|_| "Data deserialization failed".to_string())
    } else {
        Err("No data in response".to_string())
    }
}
