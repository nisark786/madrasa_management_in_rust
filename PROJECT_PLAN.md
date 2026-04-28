# Project Plan - Madrasa Student Management System

## 1) Vision

Build a secure, high-performance, multi-tenant student management platform for madrasa institutions using a modular monolith architecture.

Primary goals:

- Strong tenant isolation with shared DB + shared schema + PostgreSQL RLS
- Strict data integrity and relational consistency
- Full audit logging for all critical actions
- Role-based access control across student, staff, manager, and platform admin
- Fast user experience with Leptos SSR/hydration and backend optimization

## 2) Core Stack

- Backend: Rust + Axum
- Frontend: Rust + Leptos (WASM + SSR)
- DB: PostgreSQL
- Driver: SQLx
- Cache: Redis
- Auth: JWT (access + refresh rotation)
- UI: Tailwind CSS + lucide-rust
- Build/deploy: Cargo, cargo-leptos, Docker

## 3) Architecture: Modular Monolith

Single deployable service with strict module boundaries.

Planned modules:

- identity_access
- tenant_admin
- student_registry
- staff_registry
- academics
- quran_module
- finance
- communication
- reports_analytics
- audit_compliance
- platform_ops

## 4) Multi-Tenancy Model

- Shared database and shared schema
- Every tenant-owned row contains `tenant_id`
- Tenant-aware unique constraints (e.g., `tenant_id + student_code`)
- PostgreSQL RLS enabled on tenant tables
- Request-level DB session context (`app.tenant_id`, `app.user_id`, `app.role`)
- Platform admin policies allow controlled cross-tenant access

## 5) Roles and RBAC

- student
- staff
- manager
- platform_admin

Authorization layers:

- API-level permission guards
- Resource ownership checks
- DB-level RLS policies

## 6) Integrity and Audit

- FK constraints and normalized schema
- Transactional writes for multi-step operations
- Optimistic concurrency via versioning
- Immutable audit records for auth, CRUD, role changes, and exports

Audit fields include:

- actor, action, tenant, entity, before/after payload, ip, user-agent, request_id, timestamp

## 7) Feature Coverage

### Student

- Profile, attendance, timetable
- Homework and exam results
- Quran progress (hifz/tajweed views)
- Fee status and receipts
- Notifications and leave requests

### Staff

- Attendance management
- Homework and grading
- Quran assessment workflows
- Student performance notes
- Controlled communication with guardians/students

### Manager

- Admissions and approvals
- Class/staff allocation
- Timetable and conflicts
- Fee structures and waivers
- Dashboards and analytics
- Audit search and compliance reports

### Platform Admin

- Tenant lifecycle management
- Global monitoring and support tools
- Security visibility and policy operations

## 8) Performance Strategy

- Leptos SSR + hydration
- Cursor pagination for large datasets
- Redis caching with tenant-scoped keys
- SQL indexes (B-tree, GIN for search)
- Async background jobs for expensive tasks

## 9) Security

- JWT with short-lived access and rotating refresh tokens
- Argon2id password hashing
- Rate limiting and brute-force protections
- Input validation + typed errors
- No secret leakage in logs/responses

## 10) Quality Gates

- Tenant isolation tests
- RBAC policy tests
- Audit coverage checks
- Migration safety and rollback strategy
- Performance budgets (p95 latency targets)

## 11) Research Snapshot (Similar Systems)

Observed in publicly listed products focused on madrasah/islamic school management:

- attendance and timetable
- online admissions
- homework and progress tracking
- exams and reports
- fee collection and accounting
- parent communication
- quran/hifz tracking

Our differentiation:

- stronger type-safe backend in Rust
- strict RLS-based tenant isolation
- first-class audit/compliance model
- modular monolith maintainability and performance
