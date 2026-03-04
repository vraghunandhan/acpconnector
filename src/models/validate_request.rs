use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePaymentRequest {
    pub vault_token: String,
    pub amount: i64,
    pub currency: String,
    pub merchant_id: String,
    pub checkout_session_id: String,
}
