use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardNumberType {
    Fpan,
    NetworkToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardFundingType {
    Credit,
    Debit,
    Prepaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub payment_type: String,
    pub card_number_type: CardNumberType,
    pub number: String,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub name: Option<String>,
    pub cvc: Option<String>,
    pub cryptogram: Option<String>,
    pub eci_value: Option<String>,
    pub checks_performed: Option<Vec<String>>,
    pub iin: Option<String>,
    pub display_card_funding_type: Option<CardFundingType>,
    pub display_wallet_type: Option<String>,
    pub display_brand: Option<String>,
    pub display_last4: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
