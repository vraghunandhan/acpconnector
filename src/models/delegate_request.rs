use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{address::Address, allowance::Allowance, payment_method::PaymentMethod, risk_signal::RiskSignal};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatePaymentRequest {
    pub payment_method: PaymentMethod,
    pub allowance: Allowance,
    pub billing_address: Option<Address>,
    pub risk_signals: Vec<RiskSignal>,
    pub metadata: HashMap<String, serde_json::Value>,
}
