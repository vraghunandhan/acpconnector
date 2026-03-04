use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::models::{allowance::Allowance, payment_method::PaymentMethod, address::Address};

pub mod redis_store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAllowance {
    pub allowance: Allowance,
    pub payment_method: PaymentMethod,
    pub billing_address: Option<Address>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
    pub idempotency_key: Option<String>,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn store_vault_token(
        &self,
        token_id: &str,
        stored: &StoredAllowance,
    ) -> Result<(), AppError>;

    async fn get_vault_token(&self, token_id: &str) -> Result<Option<StoredAllowance>, AppError>;

    async fn mark_vault_token_used(&self, token_id: &str) -> Result<bool, AppError>;

    async fn store_idempotency_key(
        &self,
        key: &str,
        token_id: &str,
        ttl_seconds: u64,
    ) -> Result<(), AppError>;

    async fn get_idempotency_key(&self, key: &str) -> Result<Option<String>, AppError>;
}
