# Leptos Frontend Implementation

## Overview
A modern, minimal, premium-looking Leptos SPA frontend for the Madrasa Student Management System. The frontend features:

- ✨ **Modern Design**: Gradient backgrounds, smooth animations, premium aesthetics
- 🎯 **Minimal & Clean**: Focused on usability with zero clutter
- 🔐 **Token-Based Auth**: JWT stored in localStorage with automatic recovery
- 🎭 **Role-Based Dashboards**: Unique views for platform_admin, manager, staff, student
- 📱 **Responsive**: Works seamlessly on mobile and desktop
- ♿ **Accessible**: WCAG 2.1 AA compliance with proper focus management
- ⚡ **Fast**: CSR (Client-Side Rendering) with lazy loading support

---

## Architecture

### Project Structure
```
apps/web/src/
├── lib.rs                  # Main App component with routing
├── main.rs                 # Server entrypoint (SSR host)
├── auth.rs                 # Authentication context & token management
├── api.rs                  # API client for backend communication
├── components/
│   ├── mod.rs
│   ├── common.rs          # Reusable UI components (Button, Card, Input, etc.)
│   └── layouts.rs         # App layout (Sidebar, Header)
└── pages/
    ├── mod.rs
    ├── login.rs           # Login page with form
    └── dashboard.rs       # Role-based dashboards
```

### Tech Stack
- **Framework**: Leptos 0.6 (CSR mode)
- **Routing**: leptos_router
- **Styling**: Tailwind CSS 3.3
- **State**: Leptos RwSignal (reactive)
- **HTTP**: Fetch API via wasm-bindgen
- **Storage**: localStorage for auth tokens
- **Icons**: Unicode emojis (no external icon lib)

---

## Components

### Common Components (`common.rs`)

#### Button
```rust
<Button 
    variant="primary"  // primary | secondary | outline | danger
    loading=false
    disabled=false
    class="w-full"
    on_click=move |_| { /* action */ }
>
    "Click Me"
</Button>
```

#### Card
Premium shadow and border with hover effects
```rust
<Card class="lg:col-span-2">
    <h3>"Content"</h3>
</Card>
```

#### Input
Full-featured text input with validation ready
```rust
<Input
    input_type="email"
    placeholder="user@example.com"
    value=Some(email.get())
    on_input=move |v| email.set(v)
    required=true
/>
```

#### Label
Semantic form label
```rust
<Label for_id=Some("email".to_string())>
    "Email Address"
</Label>
```

#### FormGroup
Container with consistent spacing
```rust
<FormGroup>
    <Label for_id=Some("field".to_string())>"Field"</Label>
    <Input />
</FormGroup>
```

#### Badge
Status indicators with variants
```rust
<Badge variant="success">"Active"</Badge>
<Badge variant="warning">"Pending"</Badge>
<Badge variant="danger">"Failed"</Badge>
```

#### Alert
Info/warning/error/success messages
```rust
<Alert variant="error">"Something went wrong"</Alert>
```

#### Loading
Full-screen loading spinner
```rust
<Loading />
```

### Layout Components (`layouts.rs`)

#### AppLayout
Wraps authenticated pages with sidebar + header
```rust
<AppLayout>
    {children()}
</AppLayout>
```

Features:
- Responsive sidebar with role-based navigation
- Premium header with user avatar
- Smooth transitions between pages
- Dark theme sidebar with gradient

---

## Pages

### Login Page (`login.rs`)
Premium, minimal login interface

**Features:**
- Email, password, tenant slug inputs
- Form validation
- Loading state on button
- Error alerts
- Demo credentials displayed
- Responsive (mobile-first)

**Design:**
- Centered card layout
- Gradient background
- Logo with icon
- Professional typography

### Dashboard Page (`dashboard.rs`)
Role-based dashboard with quick stats and actions

**Role-Specific Views:**

1. **Platform Admin**
   - Total tenants, active users, pending invoices, system health
   - Tenant management quick actions
   - Recent activity feed

2. **Manager**
   - Student count, active classes, pending payments, monthly revenue
   - Enrollment overview by course
   - Quick actions (add student, create invoice, schedule)

3. **Staff (Teacher)**
   - My students count, sessions this month, avg tajweed score
   - Today's schedule with students and surahs
   - Quick actions (record session, add scores)

4. **Student**
   - Progress percentage, current surah, outstanding balance
   - Memorization progress tracker
   - Next session preview with teacher info

**Components on Dashboard:**
- `StatCard` - Key metrics with gradient backgrounds
- `ActionButton` - Quick action links
- `ActivityItem` - Activity feed entries
- `EnrollmentItem` - Course enrollment status
- `ScheduleItem` - Scheduled sessions
- `ProgressItem` - Progress bars with percentages

---

## Authentication

### AuthContext (`auth.rs`)

**Properties:**
```rust
pub struct AuthContext {
    pub user: RwSignal<Option<AuthUser>>,
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}
```

**Methods:**

```rust
// Login with credentials
auth.login(email, password, tenant_slug).await

// Logout (clears localStorage)
auth.logout()

// Check authentication status
auth.is_authenticated() -> bool

// Check if user can manage (manager+)
auth.can_manage() -> bool
```

**Token Storage:**
- Tokens stored in `localStorage["auth_user"]` as JSON
- Auto-loads on app startup
- Persists across page refreshes
- Cleared on logout

**Example Usage:**
```rust
let auth = use_auth().expect("Auth context");

let handle_login = move |_| {
    spawn_local(async move {
        auth.login(&email, &password, &tenant_slug).await;
        if auth.is_authenticated() {
            navigate("/", Default::default());
        }
    });
};
```

---

## API Client (`api.rs`)

### api_call Function
Generic function for all backend requests

```rust
pub async fn api_call<T>(
    method: &str,           // "GET", "POST", "PUT"
    path: &str,             // "/v1/students"
    token: Option<&str>,    // JWT token
    body: Option<String>,   // JSON body
) -> Result<T, String>
```

**Usage Examples:**

```rust
// Get students list
let response: Vec<StudentDto> = api_call(
    "GET",
    "/students?limit=20",
    Some(&auth_token),
    None
).await?;

// Create student
let body = serde_json::json!({
    "full_name": "Ahmed Ali",
    "student_code": "STU001"
});
let response: StudentDto = api_call(
    "POST",
    "/students",
    Some(&auth_token),
    Some(body.to_string())
).await?;
```

### DTOs

```rust
pub struct StudentDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub full_name: String,
    pub student_code: String,
    pub status: String,
    pub version: i64,
}

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

// ... more DTOs for Staff, FeePlan, etc.
```

---

## Styling

### Tailwind CSS
Used for all styling. Classes follow utility-first pattern.

### Custom CSS (`lib.rs`)
Minimal custom CSS for:
- Font family (Inter)
- Smooth transitions
- Scrollbar styling
- Focus states

### Design System

**Colors:**
- Primary: Blue (`from-blue-600 to-blue-700`)
- Secondary: Slate (`bg-slate-100`)
- Success: Green (`bg-green-100`)
- Warning: Amber (`bg-amber-100`)
- Danger: Red (`bg-red-100`)

**Spacing:**
- Consistent 4px-based scale
- Card padding: 24px
- Button padding: 10px (vertical) × 24px (horizontal)

**Border Radius:**
- Cards: 12px (`rounded-xl`)
- Buttons: 8px (`rounded-lg`)
- Inputs: 8px (`rounded-lg`)

**Shadows:**
- Card default: `shadow-sm`
- Card hover: `shadow-md`
- Button: `shadow-md` (primary only)

---

## User Flows

### Login Flow
```
1. User navigates to /login
2. Enters email, password, tenant slug
3. Click "Sign In"
4. AuthContext makes API call to /api/v1/identity/login
5. Backend returns access_token + refresh_token + user data
6. AuthContext saves to localStorage
7. User redirected to /
8. Dashboard loads with user's role
```

### Protected Routes
```
1. User not authenticated → Redirected to /login
2. User authenticated → AppLayout rendered with sidebar
3. Sidebar navigation shows role-specific options
4. On logout → Navigate to /login, clear localStorage
```

### Data Fetching
```
1. Component needs data
2. Create async action with api_call
3. Show Loading state
4. Display data or error alert
5. Optional: auto-refresh on interval
```

---

## Responsive Design

### Breakpoints (Tailwind)
- Mobile: < 640px
- Tablet: 640px - 1024px
- Desktop: > 1024px

### Responsive Classes
```rust
// Grid that stacks on mobile
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">

// Hide on mobile
<div class="hidden sm:block">

// Full width on mobile, fixed on desktop
<div class="w-full lg:w-64">
```

---

## Accessibility

### Keyboard Navigation
- Tab through form inputs
- Enter to submit forms
- Escape to close modals (future)

### Focus States
- Blue outline with offset
- High contrast for visibility

### ARIA Attributes
- Proper label associations
- Button types defined
- Form inputs required attributes

### Color Contrast
- All text meets WCAG AA standards
- Icons supplemented with text
- Status conveyed with text, not color alone

---

## Performance Optimizations

### Current
- CSR mode (no SSR overhead)
- Minimal JavaScript bundle
- Efficient re-renders via Leptos signals
- localStorage caching for tokens

### Future Improvements
- Lazy-load dashboard components
- Virtual scrolling for long lists
- Image compression/resizing
- Service Worker for offline support

---

## Browser Support
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari 14+, Chrome Mobile)

---

## Development

### Local Testing
```bash
# Start web server
cargo run -p madrasa-web

# Server runs on http://localhost:3000
# API proxied to http://localhost:8080
```

### Build
```bash
cargo build --release -p madrasa-web
```

### Docker
Included in `docker-compose.yml`:
- Multi-stage build for minimal image
- Served via nginx reverse proxy
- Auto-refresh on file changes (watch mode)

---

## Future Enhancements

1. **Students Page**
   - Table with search/filter
   - Create/edit student forms
   - Bulk import from CSV

2. **Staff Management**
   - Staff directory with profiles
   - Assignment to classes
   - Availability calendar

3. **Quran Tracking**
   - Hifz profile details
   - Session history with notes
   - Tajweed scores visualization
   - Progress charts

4. **Finance Dashboard**
   - Invoices list with status
   - Payment tracking
   - Fee plan management
   - Income reports

5. **Advanced Features**
   - Real-time notifications
   - Chat/messaging system
   - Document uploads
   - Email templates
   - SMS notifications
   - Analytics/reporting

---

## Deployment

### Production Build
```bash
# Create optimized Wasm bundle
cargo build --release -p madrasa-web

# Output served by nginx from /usr/share/nginx/html
```

### Environment Variables
- `WEB_PORT` - Port to listen on (default: 3000)
- `RUST_LOG` - Logging level (default: info)

### Security Headers (nginx)
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: SAMEORIGIN`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`

---

## Support & Troubleshooting

### Common Issues

**Blank page on load:**
- Check browser console for errors
- Verify API server running at http://localhost:8080
- Clear localStorage: `localStorage.clear()`

**Login fails:**
- Check email/password/tenant_slug
- Verify bootstrap was run: `curl -X POST http://localhost:8080/api/v1/identity/bootstrap ...`

**API errors:**
- Verify JWT token in localStorage
- Check token not expired
- Review API server logs

### Debug Mode
```rust
// Enable console logging
leptos_meta::provide_meta_context();

// Use web_sys::console::log_1 for debugging
web_sys::console::log_1(&"Debug message".into());
```

---

## Code Examples

### Create a Simple Form Component
```rust
#[component]
pub fn AddStudentForm() -> impl IntoView {
    let name = create_rw_signal(String::new());
    let code = create_rw_signal(String::new());
    let auth = use_auth().expect("Auth");

    let handle_submit = move |_| {
        spawn_local(async move {
            let body = serde_json::json!({
                "full_name": name.get(),
                "student_code": code.get(),
            });
            match api_call::<StudentDto>(
                "POST",
                "/students",
                Some(&auth.user.with(|u| u.as_ref().map(|u| u.access_token.clone()).unwrap_or_default())),
                Some(body.to_string())
            ).await {
                Ok(student) => {
                    // Handle success
                    name.set(String::new());
                    code.set(String::new());
                },
                Err(e) => {
                    auth.error.set(Some(e));
                }
            }
        });
    };

    view! {
        <Card>
            <h3 class="text-lg font-bold mb-4">"Add Student"</h3>
            <form on:submit=move |ev| {
                ev.prevent_default();
                handle_submit(());
            } class="space-y-5">
                <FormGroup>
                    <Label for_id=Some("name".to_string())>"Full Name"</Label>
                    <Input
                        placeholder="Student name"
                        value=Some(name.get())
                        on_input=move |v| name.set(v)
                    />
                </FormGroup>
                <FormGroup>
                    <Label for_id=Some("code".to_string())>"Student Code"</Label>
                    <Input
                        placeholder="STU001"
                        value=Some(code.get())
                        on_input=move |v| code.set(v)
                    />
                </FormGroup>
                <Button variant="primary" class="w-full">
                    "Add Student"
                </Button>
            </form>
        </Card>
    }
}
```

---

## Summary

The Leptos frontend provides a **modern, premium-looking, minimal UI** with:
- ✨ Beautiful gradients and smooth animations
- 🎯 Focused, clutter-free interface
- 🔐 Secure token management
- 🎭 Role-specific experiences
- ⚡ Fast, responsive performance
- ♿ Accessible & inclusive design

Perfect for a professional madrasa management platform!
