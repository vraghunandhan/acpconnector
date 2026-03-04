use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String, param: Option<String> },

    #[error("Invalid card: {message}")]
    InvalidCard { message: String, param: Option<String> },

    #[error("Idempotency conflict")]
    IdempotencyConflict,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found")]
    NotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    #[serde(rename = "type")]
    error_type: String,
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    param: Option<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, code, message, param) = match self {
            AppError::InvalidRequest { message, param } => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "invalid_request",
                message.clone(),
                param.clone(),
            ),
            AppError::InvalidCard { message, param } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_request",
                "invalid_card",
                message.clone(),
                param.clone(),
            ),
            AppError::IdempotencyConflict => (
                StatusCode::CONFLICT,
                "idempotency_conflict",
                "idempotency_conflict",
                "Same idempotency key with different parameters".to_string(),
                None,
            ),
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limit_exceeded",
                "rate_limit_exceeded",
                "Too many requests".to_string(),
                None,
            ),
            AppError::ProcessingError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "processing_error",
                "processing_error",
                msg.clone(),
                None,
            ),
            AppError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "service_unavailable",
                "service_unavailable",
                "Temporary outage".to_string(),
                None,
            ),
            AppError::RedisError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "processing_error",
                "processing_error",
                format!("Storage error: {}", e),
                None,
            ),
            AppError::SerializationError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "processing_error",
                "processing_error",
                format!("Serialization error: {}", e),
                None,
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "invalid_request",
                msg.clone(),
                None,
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "invalid_request",
                "not_found",
                "Resource not found".to_string(),
                None,
            ),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error_type: error_type.to_string(),
            code: code.to_string(),
            message,
            param,
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::InvalidCard { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::IdempotencyConflict => StatusCode::CONFLICT,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::ProcessingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
