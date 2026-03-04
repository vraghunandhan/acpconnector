use actix_web::{web, HttpResponse};
use chrono::Utc;
use tracing::info;

use crate::errors::AppError;
use crate::models::{ValidatePaymentRequest, ValidatePaymentResponse};
use crate::AppState;

pub async fn validate_payment(
    state: web::Data<AppState>,
    body: web::Json<ValidatePaymentRequest>,
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    info!(
        "Validating payment for token: {}, amount: {} {}",
        request.vault_token, request.amount, request.currency
    );

    // Fetch the stored allowance
    let stored = match state.storage.get_vault_token(&request.vault_token).await? {
        Some(s) => s,
        None => {
            return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
                valid: false,
                vault_token: Some(request.vault_token),
                message: Some("Vault token not found".to_string()),
                code: Some("invalid_token".to_string()),
            }));
        }
    };

    // Check if token is expired
    let now = Utc::now();
    if stored.allowance.expires_at <= now {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some("Vault token has expired".to_string()),
            code: Some("token_expired".to_string()),
        }));
    }

    // Check if token is already used (one_time restriction)
    if stored.used {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some("Vault token has already been used".to_string()),
            code: Some("token_used".to_string()),
        }));
    }

    // Validate amount <= max_amount
    if request.amount > stored.allowance.max_amount {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some(format!(
                "Amount {} exceeds maximum allowed {}",
                request.amount, stored.allowance.max_amount
            )),
            code: Some("amount_exceeded".to_string()),
        }));
    }

    // Validate currency match
    if request.currency != stored.allowance.currency {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some(format!(
                "Currency mismatch: expected {}, got {}",
                stored.allowance.currency, request.currency
            )),
            code: Some("currency_mismatch".to_string()),
        }));
    }

    // Validate merchant_id match
    if request.merchant_id != stored.allowance.merchant_id {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some("Merchant ID does not match allowance".to_string()),
            code: Some("merchant_mismatch".to_string()),
        }));
    }

    // Validate checkout_session_id match
    if request.checkout_session_id != stored.allowance.checkout_session_id {
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some("Checkout session ID does not match allowance".to_string()),
            code: Some("session_mismatch".to_string()),
        }));
    }

    // All validations passed - mark token as used
    let marked = state.storage.mark_vault_token_used(&request.vault_token).await?;

    if !marked {
        // Token was used between our read and write (race condition)
        return Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
            valid: false,
            vault_token: Some(request.vault_token),
            message: Some("Vault token has already been used".to_string()),
            code: Some("token_used".to_string()),
        }));
    }

    info!("Payment validated successfully for token: {}", request.vault_token);

    Ok(HttpResponse::Ok().json(ValidatePaymentResponse {
        valid: true,
        vault_token: Some(request.vault_token),
        message: Some("Payment validated".to_string()),
        code: None,
    }))
}
