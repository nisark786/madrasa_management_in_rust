CREATE EXTENSION IF NOT EXISTS pgcrypto;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type t
        WHERE t.typname = 'user_role'
    ) THEN
        CREATE TYPE user_role AS ENUM ('student', 'staff', 'manager', 'platform_admin');
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, email)
);

CREATE TABLE IF NOT EXISTS student_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    user_id UUID UNIQUE REFERENCES users(id) ON DELETE SET NULL,
    student_code TEXT NOT NULL,
    full_name TEXT NOT NULL,
    date_of_birth DATE,
    admission_date DATE,
    status TEXT NOT NULL DEFAULT 'active',
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, student_code)
);

CREATE TABLE IF NOT EXISTS staff_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    user_id UUID UNIQUE REFERENCES users(id) ON DELETE SET NULL,
    employee_code TEXT NOT NULL,
    full_name TEXT NOT NULL,
    designation TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, employee_code)
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    actor_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    actor_role user_role,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id UUID,
    before_json JSONB,
    after_json JSONB,
    ip_address INET,
    user_agent TEXT,
    request_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, code)
);

CREATE TABLE IF NOT EXISTS classes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE RESTRICT,
    teacher_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    class_name TEXT NOT NULL,
    capacity INTEGER NOT NULL DEFAULT 40 CHECK (capacity > 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS enrollments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    class_id UUID NOT NULL REFERENCES classes(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    enrolled_on DATE NOT NULL DEFAULT CURRENT_DATE,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, class_id, student_profile_id)
);

CREATE TABLE IF NOT EXISTS attendance_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    class_id UUID NOT NULL REFERENCES classes(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    attendance_date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('present', 'absent', 'late', 'excused')),
    notes TEXT,
    marked_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, class_id, student_profile_id, attendance_date)
);

CREATE INDEX IF NOT EXISTS idx_users_tenant_role ON users (tenant_id, role);
CREATE INDEX IF NOT EXISTS idx_students_tenant_created ON student_profiles (tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_staff_tenant_created ON staff_profiles (tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_tenant_created ON audit_logs (tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs (action);
CREATE INDEX IF NOT EXISTS idx_audit_after_json_gin ON audit_logs USING GIN (after_json);
CREATE INDEX IF NOT EXISTS idx_courses_tenant_name ON courses (tenant_id, name);
CREATE INDEX IF NOT EXISTS idx_classes_tenant_course ON classes (tenant_id, course_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_tenant_class ON enrollments (tenant_id, class_id);
CREATE INDEX IF NOT EXISTS idx_attendance_tenant_date ON attendance_records (tenant_id, attendance_date DESC);

ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE student_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE staff_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE courses ENABLE ROW LEVEL SECURITY;
ALTER TABLE classes ENABLE ROW LEVEL SECURITY;
ALTER TABLE enrollments ENABLE ROW LEVEL SECURITY;
ALTER TABLE attendance_records ENABLE ROW LEVEL SECURITY;

CREATE POLICY users_tenant_policy ON users
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY student_profiles_tenant_policy ON student_profiles
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY staff_profiles_tenant_policy ON staff_profiles
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY audit_logs_tenant_policy ON audit_logs
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY courses_tenant_policy ON courses
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY classes_tenant_policy ON classes
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY enrollments_tenant_policy ON enrollments
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY attendance_records_tenant_policy ON attendance_records
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

-- QURAN MODULE TABLES
CREATE TABLE IF NOT EXISTS hifz_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'paused')),
    total_chapters_memorized INTEGER NOT NULL DEFAULT 0,
    total_verses_memorized INTEGER NOT NULL DEFAULT 0,
    current_juz INTEGER,
    current_surah INTEGER,
    current_verse INTEGER,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, student_profile_id)
);

CREATE TABLE IF NOT EXISTS quran_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    teacher_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    hifz_profile_id UUID NOT NULL REFERENCES hifz_profiles(id) ON DELETE RESTRICT,
    session_date DATE NOT NULL,
    session_type TEXT NOT NULL CHECK (session_type IN ('sabaq', 'sabqi', 'manzil', 'review')),
    duration_minutes INTEGER,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tajweed_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    quran_session_id UUID NOT NULL REFERENCES quran_sessions(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    teacher_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    category TEXT NOT NULL CHECK (category IN ('makhraj', 'prolongation', 'emphasis', 'stops', 'general')),
    score INTEGER NOT NULL CHECK (score >= 1 AND score <= 10),
    feedback TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- FINANCE MODULE TABLES
CREATE TABLE IF NOT EXISTS fee_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    description TEXT,
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0),
    billing_cycle TEXT NOT NULL CHECK (billing_cycle IN ('monthly', 'quarterly', 'semi_annual', 'annual')),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS student_enrollments_finance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    fee_plan_id UUID NOT NULL REFERENCES fee_plans(id) ON DELETE RESTRICT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'suspended')),
    enrollment_date DATE NOT NULL DEFAULT CURRENT_DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, student_profile_id)
);

CREATE TABLE IF NOT EXISTS invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    invoice_number TEXT NOT NULL,
    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 0),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'overdue', 'cancelled')),
    due_date DATE NOT NULL,
    issued_date DATE NOT NULL DEFAULT CURRENT_DATE,
    notes TEXT,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, invoice_number)
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE RESTRICT,
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE RESTRICT,
    student_profile_id UUID NOT NULL REFERENCES student_profiles(id) ON DELETE RESTRICT,
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0),
    payment_method TEXT NOT NULL CHECK (payment_method IN ('cash', 'bank_transfer', 'card', 'check', 'other')),
    payment_date DATE NOT NULL DEFAULT CURRENT_DATE,
    reference_number TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- INDEXES FOR QURAN & FINANCE
CREATE INDEX IF NOT EXISTS idx_hifz_profiles_tenant_student ON hifz_profiles (tenant_id, student_profile_id);
CREATE INDEX IF NOT EXISTS idx_quran_sessions_tenant_date ON quran_sessions (tenant_id, session_date DESC);
CREATE INDEX IF NOT EXISTS idx_quran_sessions_student ON quran_sessions (tenant_id, student_profile_id);
CREATE INDEX IF NOT EXISTS idx_tajweed_scores_session ON tajweed_scores (tenant_id, quran_session_id);
CREATE INDEX IF NOT EXISTS idx_fee_plans_tenant ON fee_plans (tenant_id, is_active);
CREATE INDEX IF NOT EXISTS idx_student_enrollments_finance_tenant ON student_enrollments_finance (tenant_id, student_profile_id);
CREATE INDEX IF NOT EXISTS idx_invoices_tenant_status ON invoices (tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_invoices_due_date ON invoices (tenant_id, due_date);
CREATE INDEX IF NOT EXISTS idx_payments_tenant_date ON payments (tenant_id, payment_date DESC);

-- RLS POLICIES FOR QURAN MODULE
ALTER TABLE hifz_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE quran_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE tajweed_scores ENABLE ROW LEVEL SECURITY;

CREATE POLICY hifz_profiles_tenant_policy ON hifz_profiles
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY quran_sessions_tenant_policy ON quran_sessions
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY tajweed_scores_tenant_policy ON tajweed_scores
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

-- RLS POLICIES FOR FINANCE MODULE
ALTER TABLE fee_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE student_enrollments_finance ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE payments ENABLE ROW LEVEL SECURITY;

CREATE POLICY fee_plans_tenant_policy ON fee_plans
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY student_enrollments_finance_tenant_policy ON student_enrollments_finance
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY invoices_tenant_policy ON invoices
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );

CREATE POLICY payments_tenant_policy ON payments
    USING (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    )
    WITH CHECK (
        current_setting('app.role', true) = 'platform_admin'
        OR tenant_id = NULLIF(current_setting('app.tenant_id', true), '')::UUID
    );
