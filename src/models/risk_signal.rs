use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskAction {
    Blocked,
    ManualReview,
    Authorized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSignal {
    #[serde(rename = "type")]
    pub signal_type: String,
    pub score: i32,
    pub action: RiskAction,
}
