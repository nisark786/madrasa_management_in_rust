use leptos::*;
use leptos_router::use_navigate;
use crate::auth::use_auth;
use crate::components::{Button, Card, FormGroup, Input, Label, Alert};

#[component]
pub fn LoginPage() -> impl IntoView {
    let email = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let tenant_slug = create_rw_signal("test-madrasa".to_string());
    let navigate = use_navigate();
    let auth = use_auth().expect("Auth context not found");

    let handle_login = move |_| {
        let e = email.get();
        let p = password.get();
        let t = tenant_slug.get();

        if e.is_empty() || p.is_empty() || t.is_empty() {
            auth.error.set(Some("Please fill in all fields".to_string()));
            return;
        }

        spawn_local(async move {
            auth.login(&e, &p, &t).await;

            // Wait a moment for state to update
            set_timeout(
                move || {
                    if auth.is_authenticated() {
                        navigate("/", Default::default());
                    }
                },
                std::time::Duration::from_millis(300),
            );
        });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-slate-100 flex items-center justify-center px-4">
            <div class="w-full max-w-md">
                {/* Logo Section */}
                <div class="text-center mb-12">
                    <div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-blue-600 to-blue-700 rounded-2xl shadow-lg mb-4">
                        <span class="text-2xl">📖</span>
                    </div>
                    <h1 class="text-4xl font-bold text-slate-900 mb-2">"Madrasa"</h1>
                    <p class="text-slate-600">"Student Management System"</p>
                </div>

                {/* Card */}
                <Card class="shadow-xl">
                    <h2 class="text-2xl font-bold text-slate-900 mb-6">"Welcome Back"</h2>

                    {/* Error Alert */}
                    {auth.error.with(|err| {
                        err.as_ref().map(|e| {
                            view! {
                                <Alert variant="error">{e.clone()}</Alert>
                            }
                        })
                    })}

                    {/* Form */}
                    <form
                        on:submit=move |ev| {
                            ev.prevent_default();
                            handle_login(());
                        }
                        class="space-y-5 mt-6"
                    >
                        <FormGroup>
                            <Label for_id=Some("email".to_string())>"Email Address"</Label>
                            <Input
                                input_type="email"
                                placeholder="admin@madrasa.local"
                                value=Some(email.get())
                                on_input=move |v| email.set(v)
                                required=true
                            />
                        </FormGroup>

                        <FormGroup>
                            <Label for_id=Some("password".to_string())>"Password"</Label>
                            <Input
                                input_type="password"
                                placeholder="••••••••"
                                value=Some(password.get())
                                on_input=move |v| password.set(v)
                                required=true
                            />
                        </FormGroup>

                        <FormGroup>
                            <Label for_id=Some("tenant".to_string())>"Tenant Slug"</Label>
                            <Input
                                input_type="text"
                                placeholder="test-madrasa"
                                value=Some(tenant_slug.get())
                                on_input=move |v| tenant_slug.set(v)
                                required=true
                            />
                        </FormGroup>

                        <Button
                            variant="primary"
                            class="w-full mt-6"
                            loading=auth.is_loading.get()
                            on_click=move |_| handle_login(())
                        >
                            {if auth.is_loading.get() { "Signing in..." } else { "Sign In" }}
                        </Button>
                    </form>

                    {/* Demo Info */}
                    <div class="mt-8 p-4 bg-blue-50 rounded-lg border border-blue-100">
                        <p class="text-xs text-slate-600 mb-3 font-600">"Demo Credentials:"</p>
                        <div class="text-xs text-slate-700 space-y-1 font-mono">
                            <p>"📧 Email: admin@test.local"</p>
                            <p>"🔐 Password: SecurePass123"</p>
                            <p>"🏢 Tenant: test-madrasa"</p>
                        </div>
                    </div>
                </Card>

                {/* Footer */}
                <div class="text-center mt-8">
                    <p class="text-slate-600 text-sm">
                        "© 2026 Madrasa Management. All rights reserved."
                    </p>
                </div>
            </div>
        </div>
    }
}
