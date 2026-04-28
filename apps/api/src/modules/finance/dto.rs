use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeePlanRequest {
    #[validate(length(min = 2, max = 128))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(range(min = 1))]
    pub amount_cents: i64,
    pub billing_cycle: String, // monthly, quarterly, semi_annual, annual
}

#[derive(Debug, Serialize)]
pub struct FeePlanResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInvoiceRequest {
    pub student_id: Uuid,
    #[validate(range(min = 0))]
    pub amount_cents: i64,
    pub due_date: chrono::NaiveDate,
    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    pub invoice_id: Uuid,
    #[validate(range(min = 1))]
    pub amount_cents: i64,
    pub payment_method: String, // cash, bank_transfer, card, check, other
    #[validate(length(max = 50))]
    pub reference_number: Option<String>,
    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub student_profile_id: Uuid,
    pub amount_cents: i64,
    pub payment_method: String,
    pub payment_date: chrono::NaiveDate,
    pub reference_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListInvoicesResponse {
    pub items: Vec<InvoiceResponse>,
    pub total_outstanding_cents: i64,
}

#[derive(Debug, Serialize)]
pub struct InvoiceDetailsResponse {
    pub invoice: InvoiceResponse,
    pub payments: Vec<PaymentResponse>,
    pub paid_amount_cents: i64,
    pub remaining_balance_cents: i64,
}
