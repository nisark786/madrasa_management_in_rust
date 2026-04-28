use leptos::*;
use leptos_router::*;

mod auth;
mod api;
mod pages;
mod components;

use auth::AuthProvider;
use pages::{login::LoginPage, dashboard::DashboardPage};

#[component]
pub fn App() -> impl IntoView {
    // Setup meta tags
    leptos_meta::provide_meta_context();

    view! {
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>"Madrasa Student Management"</title>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
        <link href="https://cdn.jsdelivr.net/npm/tailwindcss@3.3.0/tailwind.min.css" rel="stylesheet" />
        <style>
            {r#"
            * {
                font-family: 'Inter', system-ui, -apple-system, sans-serif;
            }
            
            body {
                @apply bg-gradient-to-br from-slate-50 via-slate-50 to-blue-50;
                color: #1e293b;
                margin: 0;
                padding: 0;
            }
            
            /* Smooth transitions */
            button, a, [role="button"] {
                @apply transition-all duration-200 ease-in-out;
            }
            
            /* Focus styles for accessibility */
            :focus-visible {
                @apply outline-offset-2 outline-blue-500;
            }
            
            /* Scrollbar styling */
            ::-webkit-scrollbar {
                width: 8px;
                height: 8px;
            }
            
            ::-webkit-scrollbar-track {
                @apply bg-slate-100;
            }
            
            ::-webkit-scrollbar-thumb {
                @apply bg-slate-300 rounded-full;
            }
            
            ::-webkit-scrollbar-thumb:hover {
                @apply bg-slate-400;
            }
            "#}
        </style>

        <AuthProvider>
            <Router fallback=|| view! {
                <div class="flex items-center justify-center h-screen text-slate-600">"Page not found"</div>
            }.into_view()>
                <Routes>
                    <Route path="/login" view=LoginPage />
                    <Route path="/" view=DashboardPage />
                    <Route path="/*" view=|| view! { <div class="flex items-center justify-center h-screen">"Not Found"</div> } />
                </Routes>
            </Router>
        </AuthProvider>
    }
}
