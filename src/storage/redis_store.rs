use std::sync::Arc;

use redis::{AsyncCommands, Client};

use super::{Storage, StoredAllowance};
use crate::errors::AppError;

pub struct RedisStorage {
    client: Client,
}

impl RedisStorage {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::ProcessingError(format!("Failed to connect to Redis: {}", e)))?;
        Ok(Self { client })
    }

    fn vault_key(token_id: &str) -> String {
        format!("vault:{}", token_id)
    }

    fn idempotency_key(key: &str) -> String {
        format!("idempotency:{}", key)
    }
}

#[async_trait::async_trait]
impl Storage for RedisStorage {
    async fn store_vault_token(
        &self,
        token_id: &str,
        stored: &StoredAllowance,
    ) -> Result<(), AppError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::RedisError(e))?;

        let key = Self::vault_key(token_id);
        let value = serde_json::to_string(stored)?;

        // Calculate TTL based on expires_at
        let now = chrono::Utc::now();
        let ttl_seconds = if stored.allowance.expires_at > now {
            (stored.allowance.expires_at - now).num_seconds().max(1) as u64
        } else {
            60u64 // Minimum 60 seconds if already expired
        };

        redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl_seconds)
            .arg(&value)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::RedisError(e))?;

        Ok(())
    }

    async fn get_vault_token(&self, token_id: &str) -> Result<Option<StoredAllowance>, AppError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::RedisError(e))?;

        let key = Self::vault_key(token_id);
        let value: Option<String> = conn.get(&key).await.map_err(|e| AppError::RedisError(e))?;

        match value {
            Some(json_str) => {
                let stored: StoredAllowance = serde_json::from_str(&json_str)?;
                Ok(Some(stored))
            }
            None => Ok(None),
        }
    }

    async fn mark_vault_token_used(&self, token_id: &str) -> Result<bool, AppError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::RedisError(e))?;

        let key = Self::vault_key(token_id);

        // Get current TTL to preserve it
        let ttl: i64 = conn.ttl(&key).await.unwrap_or(3600);

        let value: Option<String> = conn.get(&key).await.map_err(|e| AppError::RedisError(e))?;

        match value {
            Some(json_str) => {
                let mut stored: StoredAllowance = serde_json::from_str(&json_str)?;

                if stored.used {
                    return Ok(false); // Already used
                }

                stored.used = true;
                let new_value = serde_json::to_string(&stored)?;

                redis::cmd("SETEX")
                    .arg(&key)
                    .arg(ttl.max(1) as u64)
                    .arg(&new_value)
                    .query_async::<_, ()>(&mut conn)
                    .await
                    .map_err(|e| AppError::RedisError(e))?;

                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn store_idempotency_key(
        &self,
        key: &str,
        token_id: &str,
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::RedisError(e))?;

        let redis_key = Self::idempotency_key(key);

        redis::cmd("SETEX")
            .arg(&redis_key)
            .arg(ttl_seconds)
            .arg(token_id)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::RedisError(e))?;

        Ok(())
    }

    async fn get_idempotency_key(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::RedisError(e))?;

        let redis_key = Self::idempotency_key(key);
        let value: Option<String> = conn.get(&redis_key).await.map_err(|e| AppError::RedisError(e))?;

        Ok(value)
    }
}

pub type StorageRef = Arc<dyn Storage>;
