# Madrasa Student Management System

Rust-first modular monolith for a multi-tenant madrasa student management platform.

## Tech stack

- Backend: Axum
- Frontend: Leptos (SSR + hydration target)
- Database: PostgreSQL + SQLx
- Cache: Redis
- Auth: JWT
- Styling: Tailwind CSS
- Icons: lucide-rust
- Build: Cargo / cargo-leptos (planned)
- Deployment: Docker

## Current status

Initial project foundation is created:

- Workspace structure
- Core architecture docs
- API bootstrap with health endpoints and DB/Redis wiring
- Domain modules scaffold
- Shared types crate
- DB crate repositories and models
- Initial SQL migration with tenant isolation and RLS policies
- JWT register/login/refresh flow with Redis-backed refresh token rotation
- Student create/list endpoints with role checks and pagination

## Run prerequisites

- Rust toolchain (`cargo`, `rustc`)
- Docker + Docker Compose

## Local infrastructure

```bash
docker compose up -d postgres redis
```

## API bootstrap flow

1. Create first tenant + platform admin user:

```bash
curl -X POST http://localhost:8080/api/v1/identity/bootstrap \
  -H "Content-Type: application/json" \
  -d '{"tenant_name":"Darul Uloom","tenant_slug":"darul-uloom","email":"admin@darul.test","password":"StrongPass123"}'
```

2. Login:

```bash
curl -X POST http://localhost:8080/api/v1/identity/login \
  -H "Content-Type: application/json" \
  -d '{"tenant_slug":"darul-uloom","email":"admin@darul.test","password":"StrongPass123"}'
```

3. Use `access_token` to call secured endpoints, and `refresh_token` with `/api/v1/identity/refresh`.

## Next implementation steps

1. Install Rust toolchain and run `cargo check`
2. Add `sqlx` offline checks and integration tests
3. Enforce RLS context per request transaction
4. Expand manager/staff workflows (attendance, classes, exams, quran tracking)
