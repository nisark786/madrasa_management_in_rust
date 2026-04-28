# Detailed Implementation Plan

## Phase 0 - Foundation (Week 1)

- Create Rust workspace and module boundaries
- Create Docker Compose (Postgres + Redis)
- Setup baseline config and environment model
- Add tracing and request id propagation
- Define unified API error contract

Deliverables:

- Compilable API skeleton
- Initial migration folder
- Architecture and coding conventions docs

## Phase 1 - Identity, Tenant, RBAC (Weeks 2-3)

- User model, role model, tenant model
- JWT access/refresh flow with rotation
- Login/logout/refresh endpoints
- Request middleware for auth context
- RLS session context propagation to SQL
- Permission checks per endpoint

Deliverables:

- Auth endpoints and tests
- Tenant-safe access baseline
- RBAC matrix documented and enforced

## Phase 2 - Student and Staff Registry (Weeks 4-5)

- Student admission and profile management
- Guardian linkage and contact data
- Staff profile and assignment
- Document metadata storage model
- Search/filter/pagination for registries

Deliverables:

- CRUD endpoints with validation
- Cursor pagination contracts
- Audit logs for all mutations

## Phase 3 - Academics and Quran Module (Weeks 6-8)

- Class, subject, term, timetable
- Attendance tracking
- Homework and exam scores
- Quran progress: sabaq/sabqi/manzil, tajweed rubric
- Student and staff dashboards

Deliverables:

- Role-specific workflows for student/staff/manager
- Performance-tuned list/read endpoints
- Conflict-safe update semantics

## Phase 4 - Finance and Communication (Weeks 9-10)

- Fee structures, invoices, discounts, waivers
- Payment records and receipts
- Announcement and notification service
- Template-based messaging integrations

Deliverables:

- Finance reports
- Reminder automation hooks
- Full audit and permission controls

## Phase 5 - Reporting, Caching, Hardening (Weeks 11-12)

- Analytics dashboards and exports
- Redis caching policy and invalidation events
- Security hardening and abuse protections
- Backup/restore runbook and observability dashboards

Deliverables:

- SLO-aligned telemetry
- Security checklist completion
- Production readiness checklist

## Domain Data Model (Initial)

Core entities:

- tenants
- users
- student_profiles
- staff_profiles
- courses
- classes
- enrollments
- attendance_records
- assessments
- grades
- fee_plans
- invoices
- payments
- announcements
- audit_logs

## API Conventions

- Versioned path: `/api/v1`
- Envelope success format for lists and single resources
- Error format:
  - `code`
  - `message`
  - `field_errors`
  - `request_id`
- Cursor pagination:
  - `items`
  - `next_cursor`
  - `has_more`

## Security Checklist

- JWT signing key rotation strategy
- Password policy + Argon2id
- Rate limit + lockout strategy
- CORS and CSRF policy
- Secure cookie flags for refresh token
- Immutable audit policy

## Delivery Cadence

- Sprint length: 2 weeks
- Release train: every sprint to staging
- UAT gates: manager + staff representative review
- Regression suite required before production release

## Definition of Done

- Endpoint has RBAC + validation + audit logging
- Endpoint tested for tenant isolation
- Observability instrumentation present
- Documentation updated
