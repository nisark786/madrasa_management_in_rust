mod dto;

use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Json, Router,
};
use db::repositories::{
    create_audit_log, create_fee_plan, create_invoice, create_payment, get_invoice_by_id,
    list_fee_plans, list_invoices, list_payments, update_invoice_status,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::app::AppState;
use crate::auth::AuthContext;
use crate::error::AppError;
use dto::{
    CreateFeePlanRequest, CreateInvoiceRequest, CreatePaymentRequest, FeePlanResponse,
    InvoiceDetailsResponse, InvoiceResponse, ListInvoicesResponse, PaymentResponse,
};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "finance_ok" }))
        .route("/fee-plans", post(create_fee_plan_handler))
        .route("/fee-plans", get(list_fee_plans_handler))
        .route("/invoices", post(create_invoice_handler))
        .route("/invoices", get(list_invoices_handler))
        .route("/invoices/:invoice_id", get(get_invoice_handler))
        .route("/invoices/:invoice_id/payments", post(create_payment_handler))
}

async fn create_fee_plan_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateFeePlanRequest>,
) -> Result<Json<FeePlanResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let plan = create_fee_plan(
        &state.pg_pool,
        auth.tenant_id,
        &payload.name,
        payload.description.as_deref(),
        payload.amount_cents,
        &payload.billing_cycle,
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("fee_plans_tenant_id_name_key") {
            AppError::Conflict("fee plan name already exists in tenant")
        } else {
            AppError::Db
        }
    })?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "finance.fee_plan.create",
        "fee_plan",
        Some(plan.id),
    )
    .await;

    Ok(Json(FeePlanResponse {
        id: plan.id,
        tenant_id: plan.tenant_id,
        name: plan.name,
        description: plan.description,
        amount_cents: plan.amount_cents,
        billing_cycle: plan.billing_cycle,
        is_active: plan.is_active,
    }))
}

async fn list_fee_plans_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
) -> Result<Json<Vec<FeePlanResponse>>, AppError> {
    let plans = list_fee_plans(&state.pg_pool, auth.tenant_id)
        .await
        .map_err(|_| AppError::Db)?;

    let items = plans
        .into_iter()
        .map(|p| FeePlanResponse {
            id: p.id,
            tenant_id: p.tenant_id,
            name: p.name,
            description: p.description,
            amount_cents: p.amount_cents,
            billing_cycle: p.billing_cycle,
            is_active: p.is_active,
        })
        .collect();

    Ok(Json(items))
}

async fn create_invoice_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Json(payload): Json<CreateInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Generate invoice number (timestamp + random)
    let invoice_number = format!(
        "INV-{}-{}",
        chrono::Utc::now().format("%Y%m%d"),
        uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    let invoice = create_invoice(
        &state.pg_pool,
        auth.tenant_id,
        payload.student_id,
        &invoice_number,
        payload.amount_cents,
        payload.due_date,
        payload.notes.as_deref(),
    )
    .await
    .map_err(|_| AppError::Db)?;

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "finance.invoice.create",
        "invoice",
        Some(invoice.id),
    )
    .await;

    Ok(Json(InvoiceResponse {
        id: invoice.id,
        tenant_id: invoice.tenant_id,
        student_profile_id: invoice.student_profile_id,
        invoice_number: invoice.invoice_number,
        amount_cents: invoice.amount_cents,
        status: invoice.status,
        due_date: invoice.due_date,
        issued_date: invoice.issued_date,
        notes: invoice.notes,
    }))
}

#[derive(Debug, Deserialize)]
pub struct ListInvoicesQuery {
    pub status: Option<String>,
}

async fn list_invoices_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Query(_query): Query<ListInvoicesQuery>,
) -> Result<Json<ListInvoicesResponse>, AppError> {
    let invoices = list_invoices(&state.pg_pool, auth.tenant_id, 1000)
        .await
        .map_err(|_| AppError::Db)?;

    let total_outstanding = invoices
        .iter()
        .filter(|inv| inv.status != "paid" && inv.status != "cancelled")
        .map(|inv| inv.amount_cents)
        .sum();

    let items = invoices
        .into_iter()
        .map(|inv| InvoiceResponse {
            id: inv.id,
            tenant_id: inv.tenant_id,
            student_profile_id: inv.student_profile_id,
            invoice_number: inv.invoice_number,
            amount_cents: inv.amount_cents,
            status: inv.status,
            due_date: inv.due_date,
            issued_date: inv.issued_date,
            notes: inv.notes,
        })
        .collect();

    Ok(Json(ListInvoicesResponse {
        items,
        total_outstanding_cents: total_outstanding,
    }))
}

async fn get_invoice_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(invoice_id): Path<Uuid>,
) -> Result<Json<InvoiceDetailsResponse>, AppError> {
    let invoice = get_invoice_by_id(&state.pg_pool, auth.tenant_id, invoice_id)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    let payments = list_payments(&state.pg_pool, auth.tenant_id, invoice_id)
        .await
        .map_err(|_| AppError::Db)?;

    let paid_amount: i64 = payments.iter().map(|p| p.amount_cents).sum();
    let remaining = (invoice.amount_cents - paid_amount).max(0);

    let payment_responses = payments
        .into_iter()
        .map(|p| PaymentResponse {
            id: p.id,
            invoice_id: p.invoice_id,
            student_profile_id: p.student_profile_id,
            amount_cents: p.amount_cents,
            payment_method: p.payment_method,
            payment_date: p.payment_date,
            reference_number: p.reference_number,
        })
        .collect();

    Ok(Json(InvoiceDetailsResponse {
        invoice: InvoiceResponse {
            id: invoice.id,
            tenant_id: invoice.tenant_id,
            student_profile_id: invoice.student_profile_id,
            invoice_number: invoice.invoice_number,
            amount_cents: invoice.amount_cents,
            status: invoice.status,
            due_date: invoice.due_date,
            issued_date: invoice.issued_date,
            notes: invoice.notes,
        },
        payments: payment_responses,
        paid_amount_cents: paid_amount,
        remaining_balance_cents: remaining,
    }))
}

async fn create_payment_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    auth: AuthContext,
    Path(invoice_id): Path<Uuid>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<PaymentResponse>, AppError> {
    auth.require_manager_or_admin()?;
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let invoice = get_invoice_by_id(&state.pg_pool, auth.tenant_id, invoice_id)
        .await
        .map_err(|_| AppError::Db)?
        .ok_or(AppError::NotFound)?;

    let payment = create_payment(
        &state.pg_pool,
        auth.tenant_id,
        invoice_id,
        invoice.student_profile_id,
        payload.amount_cents,
        &payload.payment_method,
        payload.reference_number.as_deref(),
        payload.notes.as_deref(),
    )
    .await
    .map_err(|_| AppError::Db)?;

    // Auto-update invoice status if fully paid
    let total_payments = list_payments(&state.pg_pool, auth.tenant_id, invoice_id)
        .await
        .map_err(|_| AppError::Db)?;

    let total_paid: i64 = total_payments.iter().map(|p| p.amount_cents).sum();
    if total_paid >= invoice.amount_cents && invoice.status != "paid" {
        let _ = update_invoice_status(&state.pg_pool, auth.tenant_id, invoice_id, "paid")
            .await;
    }

    let _ = create_audit_log(
        &state.pg_pool,
        auth.tenant_id,
        Some(auth.user_id),
        Some(auth.role.clone()),
        "finance.payment.create",
        "payment",
        Some(payment.id),
    )
    .await;

    Ok(Json(PaymentResponse {
        id: payment.id,
        invoice_id: payment.invoice_id,
        student_profile_id: payment.student_profile_id,
        amount_cents: payment.amount_cents,
        payment_method: payment.payment_method,
        payment_date: payment.payment_date,
        reference_number: payment.reference_number,
    }))
}
