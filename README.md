# ACP Connector

Agentic Commerce Payment connector implementing the OpenAI Payment API specification.

## APIs Implemented

### 1. POST /agentic_commerce/delegate_payment

Creates a vault token with payment allowance constraints.

**Headers:**
- `Authorization: Bearer {api_key}` (required)
- `Idempotency-Key: {key}` (optional)
- `Content-Type: application/json`

**Request Body:**
```json
{
  "payment_method": {
    "type": "card",
    "card_number_type": "fpan",
    "number": "4111111111111111",
    "exp_month": "12",
    "exp_year": "2025",
    "cvc": "123",
    "metadata": {}
  },
  "allowance": {
    "reason": "one_time",
    "max_amount": 10000,
    "currency": "usd",
    "checkout_session_id": "sess_123",
    "merchant_id": "merchant_456",
    "expires_at": "2025-12-31T23:59:59Z"
  },
  "risk_signals": [
    {
      "type": "card_testing",
      "score": 10,
      "action": "authorized"
    }
  ],
  "metadata": {}
}
```

**Response (201):**
```json
{
  "id": "vt_550e8400-e29b-41d4-a716-446655440000",
  "created": "2025-03-04T10:30:00Z",
  "metadata": {}
}
```

### 2. POST /agentic_commerce/validate_payment

Validates a vault token against order details and constraints.

**Request Body:**
```json
{
  "vault_token": "vt_550e8400-e29b-41d4-a716-446655440000",
  "amount": 5000,
  "currency": "usd",
  "merchant_id": "merchant_456",
  "checkout_session_id": "sess_123"
}
```

**Response (200 on success):**
```json
{
 "valid": true,
  "vault_token": "vt_550e8400-e29b-41d4-a716-446655440000",
  "message": "Payment validated"
}
```

**Response (200 on failure):**
```json
{
  "valid": false,
  "vault_token": "vt_550e8400-e29b-41d4-a716-446655440000",
  "code": "amount_exceeded",
  "message": "Amount 15000 exceeds maximum allowed 10000"
}
```

## Running the Service

### Prerequisites

- Rust 1.78+ (stable)
- Redis 6.0+ running locally or via Docker

### Start Redis

```bash
docker run -d -p 6379:6379 --name acp-redis redis:7-alpine
```

### Run the Server

```bash
# Default: runs on 127.0.0.1:8080, connects to redis://127.0.0.1:6379
cargo run

# With custom configuration:
REDIS_URL=redis://localhost:6379 BIND_ADDRESS=0.0.0.0:3000 cargo run
```

## Testing the APIs

### Test Delegate Payment

```bash
curl -X POST http://localhost:8080/agentic_commerce/delegate_payment \
  -H "Authorization: Bearer test_api_key" \
  -H "Content-Type: application/json" \
  -H "Idempotency-Key: test_key_123" \
  -d '{
    "payment_method": {
      "type": "card",
      "card_number_type": "fpan",
      "number": "4111111111111111",
      "exp_month": "12",
      "exp_year": "2027",
      "cvc": "123",
      "metadata": {}
    },
    "allowance": {
      "reason": "one_time",
      "max_amount": 10000,
      "currency": "usd",
      "checkout_session_id": "sess_123",
      "merchant_id": "merchant_456",
      "expires_at": "2025-12-31T23:59:59Z"
    },
    "risk_signals": [
      {
        "type": "card_testing",
        "score": 10,
        "action": "authorized"
      }
    ],
    "metadata": {"order_id": "order_789"}
  }'
```

### Test Validate Payment

```bash
curl -X POST http://localhost:8080/agentic_commerce/validate_payment \
  -H "Content-Type: application/json" \
  -d '{
    "vault_token": "vt_<TOKEN_FROM_ABOVE>",
    "amount": 5000,
    "currency": "usd",
    "merchant_id": "merchant_456",
    "checkout_session_id": "sess_123"
  }'
```

## Validation Rules

The validate_payment endpoint checks:
1. **Token exists** - Returns `invalid_token` if not found
2. **Not expired** - Returns `token_expired` if past expiry
3. **Not used** - Returns `token_used` if already consumed (one-time only)
4. **Amount ≤ max_amount** - Returns `amount_exceeded` if over limit
5. **Currency match** - Returns `currency_mismatch` if different
6. **Merchant match** - Returns `merchant_mismatch` if different
7. **Session match** - Returns `session_mismatch` if different

## Project Structure

```
src/
├── main.rs              # Server entry point
├── errors.rs            # Error types and responses
├── models/              # Request/response structs
│   ├── address.rs
│   ├── allowance.rs
│   ├── delegate_request.rs
│   ├── delegate_response.rs
│   ├── payment_method.rs
│   ├── risk_signal.rs
│   ├── validate_request.rs
│   └── validate_response.rs
├── routes/              # API handlers
│   ├── delegate_payment.rs
│   └── validate_payment.rs
├── storage/             # Redis storage layer
│   ├── mod.rs
│   └── redis_store.rs
└── validation/          # Input validation
    └── card.rs
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REDIS_URL` | `redis://127.0.0.1:6379` | Redis connection URL |
| `BIND_ADDRESS` | `127.0.0.1:8080` | HTTP server bind address |
