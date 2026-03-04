use chrono::Datelike;

use crate::errors::AppError;
use crate::models::payment_method::PaymentMethod;

/// Validate card number using Luhn algorithm
pub fn validate_card_number(number: &str) -> Result<(), AppError> {
    // Remove any spaces or dashes
    let cleaned: String = number.chars().filter(|c| c.is_ascii_digit()).collect();

    if cleaned.len() < 13 || cleaned.len() > 19 {
        return Err(AppError::InvalidCard {
            message: "Card number must be between 13 and 19 digits".to_string(),
            param: Some("payment_method.number".to_string()),
        });
    }

    // Luhn algorithm
    let mut sum = 0;
    let mut alternate = false;

    for c in cleaned.chars().rev() {
        let mut n = c.to_digit(10).unwrap();
        if alternate {
            n *= 2;
            if n > 9 {
                n -= 9;
            }
        }
        sum += n;
        alternate = !alternate;
    }

    if sum % 10 != 0 {
        return Err(AppError::InvalidCard {
            message: "Card number failed Luhn validation".to_string(),
            param: Some("payment_method.number".to_string()),
        });
    }

    Ok(())
}

/// Validate expiry month and year
pub fn validate_expiry(exp_month: Option<&str>, exp_year: Option<&str>) -> Result<(), AppError> {
    let month_str = match exp_month {
        Some(m) => m,
        None => return Ok(()), // Optional field
    };

    let year_str = match exp_year {
        Some(y) => y,
        None => return Ok(()), // Optional field
    };

    // Parse month
    let month: u32 = month_str.parse().map_err(|_| AppError::InvalidCard {
        message: "Invalid expiry month".to_string(),
        param: Some("payment_method.exp_month".to_string()),
    })?;

    if month < 1 || month > 12 {
        return Err(AppError::InvalidCard {
            message: "Expiry month must be between 01 and 12".to_string(),
            param: Some("payment_method.exp_month".to_string()),
        });
    }

    // Parse year
    let year: i32 = year_str.parse().map_err(|_| AppError::InvalidCard {
        message: "Invalid expiry year".to_string(),
        param: Some("payment_method.exp_year".to_string()),
    })?;

    if year_str.len() != 4 {
        return Err(AppError::InvalidCard {
            message: "Expiry year must be 4 digits".to_string(),
            param: Some("payment_method.exp_year".to_string()),
        });
    }

    // Check if card is expired
    let now = chrono::Utc::now();
    let current_year = now.year();
    let current_month = now.month() as i32;

    if year < current_year || (year == current_year && month < current_month as u32) {
        return Err(AppError::InvalidCard {
            message: "Card has expired".to_string(),
            param: Some("payment_method.exp_year".to_string()),
        });
    }

    Ok(())
}

/// Validate CVC (3-4 digits)
pub fn validate_cvc(cvc: Option<&str>) -> Result<(), AppError> {
    if let Some(cvc_str) = cvc {
        if cvc_str.len() < 3 || cvc_str.len() > 4 {
            return Err(AppError::InvalidCard {
                message: "CVC must be 3 or 4 digits".to_string(),
                param: Some("payment_method.cvc".to_string()),
            });
        }
        if !cvc_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::InvalidCard {
                message: "CVC must contain only digits".to_string(),
                param: Some("payment_method.cvc".to_string()),
            });
        }
    }
    Ok(())
}

/// Validate IIN (6 digits max)
pub fn validate_iin(iin: Option<&str>) -> Result<(), AppError> {
    if let Some(iin_str) = iin {
        if iin_str.len() > 6 {
            return Err(AppError::InvalidCard {
                message: "IIN must be at most 6 digits".to_string(),
                param: Some("payment_method.iin".to_string()),
            });
        }
        if !iin_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::InvalidCard {
                message: "IIN must contain only digits".to_string(),
                param: Some("payment_method.iin".to_string()),
            });
        }
    }
    Ok(())
}

/// Validate display_last4 (4 digits max)
pub fn validate_last4(last4: Option<&str>) -> Result<(), AppError> {
    if let Some(last4_str) = last4 {
        if last4_str.len() > 4 {
            return Err(AppError::InvalidCard {
                message: "Last 4 must be at most 4 digits".to_string(),
                param: Some("payment_method.display_last4".to_string()),
            });
        }
    }
    Ok(())
}

/// Validate full payment method
pub fn validate_payment_method(pm: &PaymentMethod) -> Result<(), AppError> {
    // Validate type must be "card"
    if pm.payment_type != "card" {
        return Err(AppError::InvalidRequest {
            message: "Payment method type must be 'card'".to_string(),
            param: Some("payment_method.type".to_string()),
        });
    }

    // Validate card number
    validate_card_number(&pm.number)?;

    // Validate expiry
    validate_expiry(pm.exp_month.as_deref(), pm.exp_year.as_deref())?;

    // Validate CVC if provided
    validate_cvc(pm.cvc.as_deref())?;

    // Validate IIN if provided
    validate_iin(pm.iin.as_deref())?;

    // Validate last4 if provided
    validate_last4(pm.display_last4.as_deref())?;

    Ok(())
}
