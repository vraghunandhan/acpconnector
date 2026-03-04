use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{DelegatePaymentRequest, DelegatePaymentResponse};
use crate::storage::StoredAllowance;
use crate::validation::card::validate_payment_method;
use crate::AppState;

pub async fn delegate_payment(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DelegatePaymentRequest>,
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // Extract headers
    let idempotency_key = req
        .headers()
        .get("Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let api_key = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Validate API key presence (basic check)
    if api_key.is_none() {
        return Err(AppError::InvalidRequest {
            message: "Missing Authorization header".to_string(),
            param: None,
        });
    }

    // Check idempotency
    if let Some(ref key) = idempotency_key {
        if let Some(existing_token) = state.storage.get_idempotency_key(key).await? {
            // Return existing token
            if let Some(stored) = state.storage.get_vault_token(&existing_token).await? {
                return Ok(HttpResponse::Created().json(DelegatePaymentResponse {
                    id: existing_token,
                    created: stored.created_at,
                    metadata: stored.payment_method.metadata.clone(),
                }));
            }
        }
    }

    // Validate allowance reason
    match request.allowance.reason {
        crate::models::AllowanceReason::OneTime => {}
    }

    // Validate currency format (ISO-4217 lowercase)
    if !request.allowance.currency.chars().all(|c| c.is_ascii_lowercase()) {
        return Err(AppError::InvalidRequest {
            message: "Currency must be lowercase ISO-4217 format".to_string(),
            param: Some("allowance.currency".to_string()),
        });
    }

    // Validate merchant_id length
    if request.allowance.merchant_id.len() > 256 {
        return Err(AppError::InvalidRequest {
            message: "Merchant ID must be at most 256 characters".to_string(),
            param: Some("allowance.merchant_id".to_string()),
        });
    }

    // Validate expiry is in the future
    let now = Utc::now();
    if request.allowance.expires_at <= now {
        return Err(AppError::InvalidRequest {
            message: "Expiry must be in the future".to_string(),
            param: Some("allowance.expires_at".to_string()),
        });
    }

    // Validate payment method
    validate_payment_method(&request.payment_method)?;

    // Generate vault token
    let token_id = format!("vt_{}", Uuid::new_v4());
    info!("Creating vault token: {} for merchant: {}", token_id, request.allowance.merchant_id);

    // Store the allowance data
    let stored = StoredAllowance {
        allowance: request.allowance,
        payment_method: request.payment_method,
        billing_address: request.billing_address,
        used: false,
        created_at: now,
        idempotency_key: idempotency_key.clone(),
    };

    state.storage.store_vault_token(&token_id, &stored).await?;

    // Store idempotency key if provided
    if let Some(key) = idempotency_key {
        // Store for 24 hours
        state.storage.store_idempotency_key(&key, &token_id, 86400).await?;
    }

    // Build response
    let response = DelegatePaymentResponse {
        id: token_id,
        created: stored.created_at,
        metadata: request.metadata,
    };

    Ok(HttpResponse::Created().json(response))
}
