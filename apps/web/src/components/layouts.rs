use leptos::*;
use leptos_router::A;
use shared::auth::Role;
use crate::auth::use_auth;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    let auth = use_auth();
    
    view! {
        <div class="flex h-screen bg-slate-50">
            {if let Some(auth) = auth {
                let auth_for_sidebar = auth.clone();
                view! {
                    <Sidebar auth=auth_for_sidebar />
                    <div class="flex-1 flex flex-col overflow-hidden">
                        <Header auth=auth />
                        <main class="flex-1 overflow-auto">
                            {children()}
                        </main>
                    </div>
                }
                .into_view()
            } else {
                children().into_view()
            }}
        </div>
    }
}

#[component]
fn Sidebar(auth: crate::auth::AuthContext) -> impl IntoView {
    let user = auth.user;
    
    let is_open = create_rw_signal(true);

    view! {
        <div class="w-64 bg-gradient-to-b from-slate-900 to-slate-800 text-white flex flex-col shadow-xl">
            {/* Logo */}
            <div class="p-6 border-b border-slate-700">
                <h1 class="text-2xl font-bold bg-gradient-to-r from-blue-400 to-blue-300 bg-clip-text text-transparent">
                    "Madrasa"
                </h1>
                <p class="text-xs text-slate-400 mt-1">"Student Management"</p>
            </div>

            {/* Navigation */}
            <nav class="flex-1 overflow-y-auto p-4 space-y-2">
                {user.with(|u| {
                    u.as_ref().map(|user| {
                        match &user.role {
                            Role::PlatformAdmin | Role::Manager => {
                                view! {
                                    <NavLink href="/" icon="📊" label="Dashboard" />
                                    <NavLink href="/students" icon="👥" label="Students" />
                                    <NavLink href="/staff" icon="👨‍🏫" label="Staff" />
                                    <NavLink href="/quran" icon="📖" label="Quran" />
                                    <NavLink href="/invoices" icon="💰" label="Finance" />
                                    <NavLink href="/audit" icon="📋" label="Audit" />
                                }
                                .into_view()
                            }
                            Role::Staff => {
                                view! {
                                    <NavLink href="/" icon="📊" label="Dashboard" />
                                    <NavLink href="/students" icon="👥" label="My Students" />
                                    <NavLink href="/quran" icon="📖" label="Quran Sessions" />
                                }
                                .into_view()
                            }
                            Role::Student => {
                                view! {
                                    <NavLink href="/" icon="📊" label="My Profile" />
                                    <NavLink href="/progress" icon="📈" label="Progress" />
                                    <NavLink href="/invoices" icon="💰" label="Invoices" />
                                }
                                .into_view()
                            }
                        }
                    })
                })}
            </nav>

            {/* Footer */}
            <div class="p-4 border-t border-slate-700">
                <button
                    on:click=move |_| {
                        auth.logout();
                    }
                    class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg text-sm font-500 transition-colors"
                >
                    "Logout"
                </button>
            </div>
        </div>
    }
}

#[component]
fn NavLink(href: &'static str, icon: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            class="flex items-center gap-3 px-4 py-3 rounded-lg text-slate-300 hover:bg-slate-700 hover:text-white transition-colors group"
            active_class="bg-blue-600 text-white"
        >
            <span class="text-xl">{icon}</span>
            <span class="font-500">{label}</span>
        </A>
    }
}

#[component]
fn Header(auth: crate::auth::AuthContext) -> impl IntoView {
    view! {
        <header class="bg-white border-b border-slate-200 shadow-sm">
            <div class="px-8 py-4 flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <h1 class="text-2xl font-bold text-slate-900">"Dashboard"</h1>
                </div>
                
                <div class="flex items-center gap-4">
                    {auth.user.with(|u| {
                        u.as_ref().map(|user| {
                            view! {
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center text-white font-bold">
                                        {user.email.chars().next().unwrap_or('U')}
                                    </div>
                                    <div class="hidden sm:block">
                                        <p class="text-sm font-600 text-slate-900">{user.email.clone()}</p>
                                        <p class="text-xs text-slate-500">{format!("{:?}", user.role)}</p>
                                    </div>
                                </div>
                            }
                        })
                    })}
                </div>
            </div>
        </header>
    }
}
