use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowanceReason {
    OneTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allowance {
    pub reason: AllowanceReason,
    pub max_amount: i64,
    pub currency: String,
    pub checkout_session_id: String,
    pub merchant_id: String,
    pub expires_at: DateTime<Utc>,
}
