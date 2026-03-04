use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatePaymentResponse {
    pub id: String,
    pub created: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}
