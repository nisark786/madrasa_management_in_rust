use leptos::*;
use serde::{Deserialize, Serialize};
use shared::auth::Role;
use uuid::Uuid;
use web_sys::window;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: Role,
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user: RwSignal<Option<AuthUser>>,
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

pub fn use_auth() -> Option<AuthContext> {
    use_context::<AuthContext>()
}

impl AuthContext {
    pub fn new() -> Self {
        let user = RwSignal::new(Self::load_user_from_storage());
        let is_loading = RwSignal::new(false);
        let error = RwSignal::new(None);

        Self {
            user,
            is_loading,
            error,
        }
    }

    pub fn load_user_from_storage() -> Option<AuthUser> {
        window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("auth_user").ok().flatten())
            .and_then(|user_json| serde_json::from_str(&user_json).ok())
    }

    pub fn save_user_to_storage(user: &AuthUser) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(
                    "auth_user",
                    &serde_json::to_string(user).unwrap_or_default(),
                );
            }
        }
    }

    pub fn clear_storage() {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("auth_user");
            }
        }
    }

    pub async fn login(&self, email: &str, password: &str, tenant_slug: &str) {
        self.is_loading.set(true);
        self.error.set(None);

        match self.do_login(email, password, tenant_slug).await {
            Ok(user) => {
                Self::save_user_to_storage(&user);
                self.user.set(Some(user));
            }
            Err(e) => {
                self.error.set(Some(e));
            }
        }

        self.is_loading.set(false);
    }

    async fn do_login(
        &self,
        email: &str,
        password: &str,
        tenant_slug: &str,
    ) -> Result<AuthUser, String> {
        use gloo_utils::format::JsValueSerdeExt;
        use wasm_bindgen_futures::JsFuture;

        let request_body = serde_json::json!({
            "email": email,
            "password": password,
            "tenant_slug": tenant_slug,
        });

        let request = web_sys::Request::new_with_str_and_init(
            "/api/v1/identity/login",
            {
                let mut init = web_sys::RequestInit::new();
                init.method("POST");
                init.headers(
                    &web_sys::Headers::new()
                        .map_err(|_| "Failed to create headers".to_string())?,
                );
                init
            }
        )
        .map_err(|_| "Failed to create request".to_string())?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|_| "Failed to set content-type".to_string())?;

        let body = request_body.to_string();
        request
            .set_body_with_opt_str(Some(&body))
            .map_err(|_| "Failed to set request body".to_string())?;

        let window = web_sys::window().ok_or("No window object".to_string())?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| "Fetch failed".to_string())?;

        let response: web_sys::Response = response.dyn_into()
            .map_err(|_| "Response conversion failed".to_string())?;

        if !response.ok() {
            return Err("Invalid credentials or tenant".to_string());
        }

        let body = JsFuture::from(response.text().map_err(|_| "Text extraction failed".to_string())?)
            .await
            .map_err(|_| "Body extraction failed".to_string())?;

        let response_text = body.as_string().ok_or("Response not text".to_string())?;
        let response_json: serde_json::Value =
            serde_json::from_str(&response_text)
                .map_err(|_| "JSON parse failed".to_string())?;

        let data = response_json
            .get("data")
            .ok_or("No data in response".to_string())?;

        let user = AuthUser {
            user_id: Uuid::parse_str(
                data.get("user_id")
                    .and_then(|v| v.as_str())
                    .ok_or("No user_id".to_string())?,
            )
            .map_err(|_| "Invalid user_id".to_string())?,
            tenant_id: Uuid::parse_str(
                data.get("tenant_id")
                    .and_then(|v| v.as_str())
                    .ok_or("No tenant_id".to_string())?,
            )
            .map_err(|_| "Invalid tenant_id".to_string())?,
            role: match data
                .get("role")
                .and_then(|v| v.as_str())
                .ok_or("No role".to_string())?
            {
                "platform_admin" => Role::PlatformAdmin,
                "manager" => Role::Manager,
                "staff" => Role::Staff,
                "student" => Role::Student,
                _ => return Err("Invalid role".to_string()),
            },
            email: data
                .get("email")
                .and_then(|v| v.as_str())
                .ok_or("No email".to_string())?
                .to_string(),
            access_token: data
                .get("access_token")
                .and_then(|v| v.as_str())
                .ok_or("No access_token".to_string())?
                .to_string(),
            refresh_token: data
                .get("refresh_token")
                .and_then(|v| v.as_str())
                .ok_or("No refresh_token".to_string())?
                .to_string(),
        };

        Ok(user)
    }

    pub fn logout(&self) {
        Self::clear_storage();
        self.user.set(None);
        self.error.set(None);
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.with(|u| u.is_some())
    }

    pub fn can_manage(&self) -> bool {
        self.user.with(|u| {
            u.as_ref().map_or(false, |user| {
                matches!(user.role, Role::Manager | Role::PlatformAdmin)
            })
        })
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth = AuthContext::new();
    provide_context(auth);

    children()
}
