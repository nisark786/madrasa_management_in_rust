use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{
    AttendanceRecord, AuditLog, Class, Course, FeePlan, HifzProfile, Invoice, Payment,
    QuranSession, StaffProfile, StudentEnrollmentFinance, StudentProfile, TajweedScore, Tenant, User,
};
use shared::auth::Role;

pub async fn get_tenant_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
    sqlx::query_as::<_, Tenant>(
        r#"
        select id, name, slug, is_active, created_at, updated_at
        from tenants
        where slug = $1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_email(pool: &PgPool, tenant_id: Uuid, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        select id, tenant_id, email, password_hash, role, is_active, created_at, updated_at
        from users
        where tenant_id = $1 and email = $2
        "#,
    )
    .bind(tenant_id)
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_email_global(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        select id, tenant_id, email, password_hash, role, is_active, created_at, updated_at
        from users
        where email = $1
        order by created_at asc
        limit 1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, tenant_id: Uuid, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        select id, tenant_id, email, password_hash, role, is_active, created_at, updated_at
        from users
        where tenant_id = $1 and id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_student_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    student_code: &str,
    full_name: &str,
) -> Result<StudentProfile, sqlx::Error> {
    sqlx::query_as::<_, StudentProfile>(
        r#"
        insert into student_profiles (tenant_id, student_code, full_name)
        values ($1, $2, $3)
        returning id, tenant_id, user_id, student_code, full_name, status, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(student_code)
    .bind(full_name)
    .fetch_one(pool)
    .await
}

pub async fn list_student_profiles(
    pool: &PgPool,
    tenant_id: Uuid,
    limit: i64,
    cursor: Option<Uuid>,
) -> Result<Vec<StudentProfile>, sqlx::Error> {
    let mut qb = QueryBuilder::<Postgres>::new(
        r#"
        select id, tenant_id, user_id, student_code, full_name, status, version, created_at, updated_at
        from student_profiles
        where tenant_id = "#,
    );

    qb.push_bind(tenant_id);

    if let Some(id) = cursor {
        qb.push(" and id > ").push_bind(id);
    }

    qb.push(" order by id asc limit ").push_bind(limit);

    qb.build_query_as::<StudentProfile>().fetch_all(pool).await
}

pub async fn list_audit_logs(
    pool: &PgPool,
    tenant_id: Uuid,
    limit: i64,
) -> Result<Vec<AuditLog>, sqlx::Error> {
    sqlx::query_as::<_, AuditLog>(
        r#"
        select id, tenant_id, actor_user_id, actor_role, action, entity_type, entity_id, created_at
        from audit_logs
        where tenant_id = $1
        order by created_at desc
        limit $2
        "#,
    )
    .bind(tenant_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn create_audit_log(
    pool: &PgPool,
    tenant_id: Uuid,
    actor_user_id: Option<Uuid>,
    actor_role: Option<Role>,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into audit_logs (
            tenant_id,
            actor_user_id,
            actor_role,
            action,
            entity_type,
            entity_id
        ) values ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(tenant_id)
    .bind(actor_user_id)
    .bind(actor_role)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_course(
    pool: &PgPool,
    tenant_id: Uuid,
    code: &str,
    name: &str,
    description: Option<&str>,
) -> Result<Course, sqlx::Error> {
    sqlx::query_as::<_, Course>(
        r#"
        insert into courses (tenant_id, code, name, description)
        values ($1, $2, $3, $4)
        returning id, tenant_id, code, name, description, is_active, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(code)
    .bind(name)
    .bind(description)
    .fetch_one(pool)
    .await
}

pub async fn list_courses(pool: &PgPool, tenant_id: Uuid, limit: i64) -> Result<Vec<Course>, sqlx::Error> {
    sqlx::query_as::<_, Course>(
        r#"
        select id, tenant_id, code, name, description, is_active, created_at, updated_at
        from courses
        where tenant_id = $1
        order by created_at desc
        limit $2
        "#,
    )
    .bind(tenant_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn create_class(
    pool: &PgPool,
    tenant_id: Uuid,
    course_id: Uuid,
    class_name: &str,
    teacher_user_id: Option<Uuid>,
    capacity: i32,
) -> Result<Class, sqlx::Error> {
    sqlx::query_as::<_, Class>(
        r#"
        insert into classes (tenant_id, course_id, class_name, teacher_user_id, capacity)
        values ($1, $2, $3, $4, $5)
        returning id, tenant_id, course_id, teacher_user_id, class_name, capacity, is_active, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(course_id)
    .bind(class_name)
    .bind(teacher_user_id)
    .bind(capacity)
    .fetch_one(pool)
    .await
}

pub async fn mark_attendance(
    pool: &PgPool,
    tenant_id: Uuid,
    class_id: Uuid,
    student_profile_id: Uuid,
    attendance_date: chrono::NaiveDate,
    status: &str,
    notes: Option<&str>,
    marked_by_user_id: Option<Uuid>,
) -> Result<AttendanceRecord, sqlx::Error> {
    sqlx::query_as::<_, AttendanceRecord>(
        r#"
        insert into attendance_records (
            tenant_id,
            class_id,
            student_profile_id,
            attendance_date,
            status,
            notes,
            marked_by_user_id
        )
        values ($1, $2, $3, $4, $5, $6, $7)
        on conflict (tenant_id, class_id, student_profile_id, attendance_date)
        do update set
            status = excluded.status,
            notes = excluded.notes,
            marked_by_user_id = excluded.marked_by_user_id,
            updated_at = now()
        returning id, tenant_id, class_id, student_profile_id, attendance_date, status, notes, marked_by_user_id, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(class_id)
    .bind(student_profile_id)
    .bind(attendance_date)
    .bind(status)
    .bind(notes)
    .bind(marked_by_user_id)
    .fetch_one(pool)
    .await
}

pub async fn create_staff_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    employee_code: &str,
    full_name: &str,
    designation: Option<&str>,
) -> Result<StaffProfile, sqlx::Error> {
    sqlx::query_as::<_, StaffProfile>(
        r#"
        insert into staff_profiles (tenant_id, employee_code, full_name, designation)
        values ($1, $2, $3, $4)
        returning id, tenant_id, user_id, employee_code, full_name, designation, status, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(employee_code)
    .bind(full_name)
    .bind(designation)
    .fetch_one(pool)
    .await
}

pub async fn list_staff_profiles(
    pool: &PgPool,
    tenant_id: Uuid,
    limit: i64,
    cursor: Option<Uuid>,
) -> Result<Vec<StaffProfile>, sqlx::Error> {
    let mut qb = QueryBuilder::<Postgres>::new(
        r#"
        select id, tenant_id, user_id, employee_code, full_name, designation, status, version, created_at, updated_at
        from staff_profiles
        where tenant_id = "#,
    );

    qb.push_bind(tenant_id);

    if let Some(id) = cursor {
        qb.push(" and id > ").push_bind(id);
    }

    qb.push(" order by id asc limit ").push_bind(limit);

    qb.build_query_as::<StaffProfile>().fetch_all(pool).await
}

pub async fn get_staff_profile_by_id(
    pool: &PgPool,
    tenant_id: Uuid,
    staff_id: Uuid,
) -> Result<Option<StaffProfile>, sqlx::Error> {
    sqlx::query_as::<_, StaffProfile>(
        r#"
        select id, tenant_id, user_id, employee_code, full_name, designation, status, version, created_at, updated_at
        from staff_profiles
        where tenant_id = $1 and id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(staff_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_staff_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    staff_id: Uuid,
    full_name: Option<&str>,
    designation: Option<&str>,
    status: Option<&str>,
) -> Result<Option<StaffProfile>, sqlx::Error> {
    let mut qb = QueryBuilder::<Postgres>::new(
        "update staff_profiles set ",
    );

    let mut first = true;

    if let Some(name) = full_name {
        if !first { qb.push(", "); }
        qb.push("full_name = ").push_bind(name);
        first = false;
    }

    if let Some(desig) = designation {
        if !first { qb.push(", "); }
        qb.push("designation = ").push_bind(desig);
        first = false;
    }

    if let Some(st) = status {
        if !first { qb.push(", "); }
        qb.push("status = ").push_bind(st);
        first = false;
    }

    if first {
        // no updates
        return get_staff_profile_by_id(pool, tenant_id, staff_id).await;
    }

    qb.push(", updated_at = now() where tenant_id = ")
        .push_bind(tenant_id)
        .push(" and id = ")
        .push_bind(staff_id)
        .push(" returning id, tenant_id, user_id, employee_code, full_name, designation, status, version, created_at, updated_at");

    qb.build_query_as::<StaffProfile>()
        .fetch_optional(pool)
        .await
}

// QURAN MODULE REPOSITORIES

pub async fn create_hifz_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    student_profile_id: Uuid,
) -> Result<HifzProfile, sqlx::Error> {
    sqlx::query_as::<_, HifzProfile>(
        r#"
        insert into hifz_profiles (tenant_id, student_profile_id)
        values ($1, $2)
        returning id, tenant_id, student_profile_id, status, total_chapters_memorized, total_verses_memorized, 
                  current_juz, current_surah, current_verse, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(student_profile_id)
    .fetch_one(pool)
    .await
}

pub async fn get_hifz_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    student_profile_id: Uuid,
) -> Result<Option<HifzProfile>, sqlx::Error> {
    sqlx::query_as::<_, HifzProfile>(
        r#"
        select id, tenant_id, student_profile_id, status, total_chapters_memorized, total_verses_memorized,
               current_juz, current_surah, current_verse, version, created_at, updated_at
        from hifz_profiles
        where tenant_id = $1 and student_profile_id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(student_profile_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_quran_session(
    pool: &PgPool,
    tenant_id: Uuid,
    student_profile_id: Uuid,
    hifz_profile_id: Uuid,
    teacher_user_id: Option<Uuid>,
    session_date: chrono::NaiveDate,
    session_type: &str,
    duration_minutes: Option<i32>,
    notes: Option<&str>,
) -> Result<QuranSession, sqlx::Error> {
    sqlx::query_as::<_, QuranSession>(
        r#"
        insert into quran_sessions (tenant_id, student_profile_id, hifz_profile_id, teacher_user_id, session_date, session_type, duration_minutes, notes)
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        returning id, tenant_id, student_profile_id, teacher_user_id, hifz_profile_id, session_date, session_type, duration_minutes, notes, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(student_profile_id)
    .bind(hifz_profile_id)
    .bind(teacher_user_id)
    .bind(session_date)
    .bind(session_type)
    .bind(duration_minutes)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn list_quran_sessions(
    pool: &PgPool,
    tenant_id: Uuid,
    student_profile_id: Uuid,
    limit: i64,
) -> Result<Vec<QuranSession>, sqlx::Error> {
    sqlx::query_as::<_, QuranSession>(
        r#"
        select id, tenant_id, student_profile_id, teacher_user_id, hifz_profile_id, session_date, session_type, duration_minutes, notes, created_at, updated_at
        from quran_sessions
        where tenant_id = $1 and student_profile_id = $2
        order by session_date desc
        limit $3
        "#,
    )
    .bind(tenant_id)
    .bind(student_profile_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn create_tajweed_score(
    pool: &PgPool,
    tenant_id: Uuid,
    quran_session_id: Uuid,
    student_profile_id: Uuid,
    teacher_user_id: Option<Uuid>,
    category: &str,
    score: i32,
    feedback: Option<&str>,
) -> Result<TajweedScore, sqlx::Error> {
    sqlx::query_as::<_, TajweedScore>(
        r#"
        insert into tajweed_scores (tenant_id, quran_session_id, student_profile_id, teacher_user_id, category, score, feedback)
        values ($1, $2, $3, $4, $5, $6, $7)
        returning id, tenant_id, quran_session_id, student_profile_id, teacher_user_id, category, score, feedback, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(quran_session_id)
    .bind(student_profile_id)
    .bind(teacher_user_id)
    .bind(category)
    .bind(score)
    .bind(feedback)
    .fetch_one(pool)
    .await
}

pub async fn list_tajweed_scores(
    pool: &PgPool,
    tenant_id: Uuid,
    quran_session_id: Uuid,
) -> Result<Vec<TajweedScore>, sqlx::Error> {
    sqlx::query_as::<_, TajweedScore>(
        r#"
        select id, tenant_id, quran_session_id, student_profile_id, teacher_user_id, category, score, feedback, created_at, updated_at
        from tajweed_scores
        where tenant_id = $1 and quran_session_id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(quran_session_id)
    .fetch_all(pool)
    .await
}

// FINANCE MODULE REPOSITORIES

pub async fn create_fee_plan(
    pool: &PgPool,
    tenant_id: Uuid,
    name: &str,
    description: Option<&str>,
    amount_cents: i64,
    billing_cycle: &str,
) -> Result<FeePlan, sqlx::Error> {
    sqlx::query_as::<_, FeePlan>(
        r#"
        insert into fee_plans (tenant_id, name, description, amount_cents, billing_cycle)
        values ($1, $2, $3, $4, $5)
        returning id, tenant_id, name, description, amount_cents, billing_cycle, is_active, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(name)
    .bind(description)
    .bind(amount_cents)
    .bind(billing_cycle)
    .fetch_one(pool)
    .await
}

pub async fn list_fee_plans(
    pool: &PgPool,
    tenant_id: Uuid,
) -> Result<Vec<FeePlan>, sqlx::Error> {
    sqlx::query_as::<_, FeePlan>(
        r#"
        select id, tenant_id, name, description, amount_cents, billing_cycle, is_active, version, created_at, updated_at
        from fee_plans
        where tenant_id = $1 and is_active = true
        order by created_at desc
        "#,
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

pub async fn create_invoice(
    pool: &PgPool,
    tenant_id: Uuid,
    student_profile_id: Uuid,
    invoice_number: &str,
    amount_cents: i64,
    due_date: chrono::NaiveDate,
    notes: Option<&str>,
) -> Result<Invoice, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        insert into invoices (tenant_id, student_profile_id, invoice_number, amount_cents, due_date, notes)
        values ($1, $2, $3, $4, $5, $6)
        returning id, tenant_id, student_profile_id, invoice_number, amount_cents, status, due_date, issued_date, notes, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(student_profile_id)
    .bind(invoice_number)
    .bind(amount_cents)
    .bind(due_date)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn list_invoices(
    pool: &PgPool,
    tenant_id: Uuid,
    limit: i64,
) -> Result<Vec<Invoice>, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        select id, tenant_id, student_profile_id, invoice_number, amount_cents, status, due_date, issued_date, notes, version, created_at, updated_at
        from invoices
        where tenant_id = $1
        order by due_date asc
        limit $2
        "#,
    )
    .bind(tenant_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_invoice_by_id(
    pool: &PgPool,
    tenant_id: Uuid,
    invoice_id: Uuid,
) -> Result<Option<Invoice>, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        select id, tenant_id, student_profile_id, invoice_number, amount_cents, status, due_date, issued_date, notes, version, created_at, updated_at
        from invoices
        where tenant_id = $1 and id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(invoice_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_payment(
    pool: &PgPool,
    tenant_id: Uuid,
    invoice_id: Uuid,
    student_profile_id: Uuid,
    amount_cents: i64,
    payment_method: &str,
    reference_number: Option<&str>,
    notes: Option<&str>,
) -> Result<Payment, sqlx::Error> {
    sqlx::query_as::<_, Payment>(
        r#"
        insert into payments (tenant_id, invoice_id, student_profile_id, amount_cents, payment_method, reference_number, notes)
        values ($1, $2, $3, $4, $5, $6, $7)
        returning id, tenant_id, invoice_id, student_profile_id, amount_cents, payment_method, payment_date, reference_number, notes, created_at
        "#,
    )
    .bind(tenant_id)
    .bind(invoice_id)
    .bind(student_profile_id)
    .bind(amount_cents)
    .bind(payment_method)
    .bind(reference_number)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn list_payments(
    pool: &PgPool,
    tenant_id: Uuid,
    invoice_id: Uuid,
) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as::<_, Payment>(
        r#"
        select id, tenant_id, invoice_id, student_profile_id, amount_cents, payment_method, payment_date, reference_number, notes, created_at
        from payments
        where tenant_id = $1 and invoice_id = $2
        order by payment_date desc
        "#,
    )
    .bind(tenant_id)
    .bind(invoice_id)
    .fetch_all(pool)
    .await
}

pub async fn update_invoice_status(
    pool: &PgPool,
    tenant_id: Uuid,
    invoice_id: Uuid,
    status: &str,
) -> Result<Option<Invoice>, sqlx::Error> {
    sqlx::query_as::<_, Invoice>(
        r#"
        update invoices
        set status = $3, updated_at = now()
        where tenant_id = $1 and id = $2
        returning id, tenant_id, student_profile_id, invoice_number, amount_cents, status, due_date, issued_date, notes, version, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(invoice_id)
    .bind(status)
    .fetch_optional(pool)
    .await
}
