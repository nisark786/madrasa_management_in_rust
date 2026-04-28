use leptos::*;
use leptos_router::use_navigate;
use shared::auth::Role;
use crate::auth::use_auth;
use crate::components::{AppLayout, Card, Badge};

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    // Check if authenticated, redirect to login if not
    create_effect(move |_| {
        if let Some(auth) = auth {
            if !auth.is_authenticated() {
                navigate("/login", Default::default());
            }
        }
    });

    let auth = auth.expect("Auth context not found");

    view! {
        <AppLayout>
            <div class="p-8">
                {auth.user.with(|u| {
                    u.as_ref().map(|user| {
                        match &user.role {
                            Role::PlatformAdmin => view! { <AdminDashboard user=user.clone() /> }.into_view(),
                            Role::Manager => view! { <ManagerDashboard user=user.clone() /> }.into_view(),
                            Role::Staff => view! { <StaffDashboard user=user.clone() /> }.into_view(),
                            Role::Student => view! { <StudentDashboard user=user.clone() /> }.into_view(),
                        }
                    })
                })}
            </div>
        </AppLayout>
    }
}

#[component]
fn AdminDashboard(user: crate::auth::AuthUser) -> impl IntoView {
    view! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-slate-900">
                    "Welcome, " {user.email}
                </h1>
                <p class="text-slate-600 mt-2">"Platform Administrator Dashboard"</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <StatCard title="Total Tenants" value="5" icon="🏢" color="blue" />
                <StatCard title="Active Users" value="48" icon="👥" color="green" />
                <StatCard title="Pending Invoices" value="12" icon="💰" color="amber" />
                <StatCard title="System Health" value="99.9%" icon="✅" color="emerald" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Quick Actions"</h3>
                    <div class="space-y-3">
                        <ActionButton label="Create Tenant" icon="➕" />
                        <ActionButton label="Manage Users" icon="👤" />
                        <ActionButton label="View Reports" icon="📊" />
                        <ActionButton label="System Settings" icon="⚙️" />
                    </div>
                </Card>

                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Recent Activity"</h3>
                    <div class="space-y-4">
                        <ActivityItem time="2 hours ago" text="New student enrolled in Quran course" />
                        <ActivityItem time="5 hours ago" text="Invoice INV-20260428-ABC1 marked as paid" />
                        <ActivityItem time="1 day ago" text="New staff member added" />
                        <ActivityItem time="2 days ago" text="Fee plan updated" />
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn ManagerDashboard(user: crate::auth::AuthUser) -> impl IntoView {
    view! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-slate-900">
                    "Welcome, " {user.email}
                </h1>
                <p class="text-slate-600 mt-2">"Manager Dashboard"</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-6 mb-8">
                <StatCard title="Total Students" value="156" icon="👥" color="blue" />
                <StatCard title="Active Classes" value="8" icon="🎓" color="purple" />
                <StatCard title="Pending Payments" value="23" icon="💰" color="amber" />
                <StatCard title="This Month Revenue" value="$4,850" icon="📈" color="green" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <Card class="lg:col-span-2">
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Enrollment Overview"</h3>
                    <div class="space-y-4">
                        <EnrollmentItem course="Quran Memorization" students="45" status="active" />
                        <EnrollmentItem course="Islamic Studies" students="32" status="active" />
                        <EnrollmentItem course="Arabic Language" students="28" status="active" />
                        <EnrollmentItem course="Tajweed Rules" students="18" status="active" />
                    </div>
                </Card>

                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Quick Actions"</h3>
                    <div class="space-y-3">
                        <ActionButton label="Add Student" icon="➕" />
                        <ActionButton label="Create Invoice" icon="📄" />
                        <ActionButton label="Schedule Class" icon="📅" />
                        <ActionButton label="View Reports" icon="📊" />
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn StaffDashboard(user: crate::auth::AuthUser) -> impl IntoView {
    view! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-slate-900">
                    "Welcome, " {user.email}
                </h1>
                <p class="text-slate-600 mt-2">"Teacher Dashboard"</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <StatCard title="My Students" value="24" icon="👨‍🎓" color="blue" />
                <StatCard title="Sessions This Month" value="18" icon="📖" color="purple" />
                <StatCard title="Avg. Tajweed Score" value="8.2/10" icon="⭐" color="amber" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Today's Schedule"</h3>
                    <div class="space-y-4">
                        <ScheduleItem time="09:00 AM" student="Ahmad Ibrahim" surah="Al-Fatiha" />
                        <ScheduleItem time="10:30 AM" student="Fatima Ahmed" surah="Al-Baqara" />
                        <ScheduleItem time="02:00 PM" student="Mohamed Ali" surah="At-Taubah" />
                        <ScheduleItem time="03:30 PM" student="Zainab Hassan" surah="Yunus" />
                    </div>
                </Card>

                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Quick Actions"</h3>
                    <div class="space-y-3">
                        <ActionButton label="Record Session" icon="📝" />
                        <ActionButton label="Add Tajweed Score" icon="⭐" />
                        <ActionButton label="View Student Progress" icon="📈" />
                        <ActionButton label="Message Students" icon="💬" />
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn StudentDashboard(user: crate::auth::AuthUser) -> impl IntoView {
    view! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-slate-900">
                    "Welcome, " {user.email}
                </h1>
                <p class="text-slate-600 mt-2">"Your Dashboard"</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <StatCard title="Progress" value="45%" icon="📊" color="green" />
                <StatCard title="Current Surah" value="Al-Baqara" icon="📖" color="blue" />
                <StatCard title="Outstanding Balance" value="$150" icon="💳" color="amber" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Your Progress"</h3>
                    <div class="space-y-4">
                        <ProgressItem label="Chapters Memorized" current="2" total="30" />
                        <ProgressItem label="Verses Memorized" current="156" total="6236" />
                        <ProgressItem label="This Month Sessions" current="12" total="16" />
                    </div>
                </Card>

                <Card>
                    <h3 class="text-lg font-bold text-slate-900 mb-4">"Next Session"</h3>
                    <div class="bg-gradient-to-br from-blue-50 to-purple-50 rounded-lg p-6 border border-blue-100">
                        <p class="text-sm text-slate-600 mb-2">"Scheduled with:"</p>
                        <p class="text-lg font-bold text-slate-900 mb-4">"Ustadh Mohammed"</p>
                        <p class="text-sm text-slate-700 mb-1">
                            "📅 Tomorrow at 10:00 AM"
                        </p>
                        <p class="text-sm text-slate-700 mb-4">
                            "📖 Surah Al-Baqara (Verses 1-50)"
                        </p>
                        <button class="w-full px-4 py-2 bg-blue-600 text-white rounded-lg text-sm font-500 hover:bg-blue-700 transition-colors">
                            "Prepare"
                        </button>
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, icon: &'static str, color: &'static str) -> impl IntoView {
    let (bg_class, border_class) = match color {
        "green" => ("bg-gradient-to-br from-green-50 to-emerald-50", "border-green-200"),
        "purple" => ("bg-gradient-to-br from-purple-50 to-pink-50", "border-purple-200"),
        "amber" => ("bg-gradient-to-br from-amber-50 to-orange-50", "border-amber-200"),
        "emerald" => ("bg-gradient-to-br from-emerald-50 to-teal-50", "border-emerald-200"),
        _ => ("bg-gradient-to-br from-blue-50 to-cyan-50", "border-blue-200"),
    };

    view! {
        <div class=format!(
            "rounded-xl p-6 border-2 {} {}",
            bg_class,
            border_class
        )>
            <div class="flex items-start justify-between">
                <div>
                    <p class="text-sm font-500 text-slate-600">{title}</p>
                    <p class="text-3xl font-bold text-slate-900 mt-2">{value}</p>
                </div>
                <span class="text-3xl">{icon}</span>
            </div>
        </div>
    }
}

#[component]
fn ActionButton(label: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <button class="w-full flex items-center gap-3 p-3 bg-slate-50 hover:bg-blue-50 border border-slate-200 hover:border-blue-300 rounded-lg transition-colors group">
            <span class="text-xl">{icon}</span>
            <span class="text-sm font-500 text-slate-700 group-hover:text-blue-600">{label}</span>
            <span class="ml-auto text-slate-400 group-hover:text-blue-400">→</span>
        </button>
    }
}

#[component]
fn ActivityItem(time: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="flex gap-3">
            <div class="w-2 h-2 bg-blue-500 rounded-full mt-1.5 flex-shrink-0"></div>
            <div>
                <p class="text-sm text-slate-700">{text}</p>
                <p class="text-xs text-slate-500 mt-0.5">{time}</p>
            </div>
        </div>
    }
}

#[component]
fn EnrollmentItem(course: &'static str, students: &'static str, status: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg">
            <div>
                <p class="font-500 text-slate-900">{course}</p>
                <p class="text-sm text-slate-600 mt-1">{students} " students"</p>
            </div>
            <Badge variant="success">{status}</Badge>
        </div>
    }
}

#[component]
fn ScheduleItem(time: &'static str, student: &'static str, surah: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-4 border border-slate-200 rounded-lg hover:bg-slate-50 transition-colors">
            <div>
                <p class="font-500 text-slate-900">{time}</p>
                <p class="text-sm text-slate-600 mt-0.5">{student}</p>
                <p class="text-xs text-blue-600 mt-1">"📖 " {surah}</p>
            </div>
            <button class="px-3 py-1 bg-blue-100 text-blue-600 rounded text-xs font-500 hover:bg-blue-200">
                "Start"
            </button>
        </div>
    }
}

#[component]
fn ProgressItem(label: &'static str, current: &'static str, total: &'static str) -> impl IntoView {
    let percentage = {
        let current_val: f32 = current.parse().unwrap_or(0.0);
        let total_val: f32 = total.parse().unwrap_or(1.0);
        (current_val / total_val * 100.0) as i32
    };

    view! {
        <div>
            <div class="flex items-center justify-between mb-2">
                <p class="text-sm font-500 text-slate-700">{label}</p>
                <p class="text-sm text-slate-600">{current} "/" {total}</p>
            </div>
            <div class="w-full bg-slate-200 rounded-full h-2">
                <div
                    class="bg-gradient-to-r from-blue-500 to-blue-600 h-2 rounded-full transition-all duration-300"
                    style=format!("width: {}%", percentage)
                />
            </div>
        </div>
    }
}
