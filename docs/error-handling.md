# Error Handling

## 1. Purpose

Errors are part of every application.

For example:
- A customer tries to access an account that does not exist.
- A customer attempts to place an invalid order.
- A product is no longer available.
- PostgreSQL is temporarily unavailable.
- An external payment provider returns an error.

The application must handle these situations consistently.

Good error handling helps us:
- Protect business rules.
- Return meaningful API responses.
- Avoid exposing internal implementation details.
- Troubleshoot problems.
- Distinguish temporary failures from permanent failures.
- Keep the application reliable.

## 2. Error Handling Principles

For `bio-com`, we follow these principles:

1. Each architectural layer defines errors related to its responsibilities.
2. Domain errors represent business rule violations.
3. Application errors represent use-case failures.
4. Infrastructure errors represent technical failures.
5. API errors represent HTTP responses.
6. Errors should be explicit and typed.
7. Internal errors must not be exposed directly to customers.
8. Errors should preserve their original cause when possible.
9. Errors should be logged at the appropriate boundary.
10. Expected business failures should not be treated as unexpected system failures.

## 3. Error Categories

We separate errors into four main categories.

| Layer | Error type | Example |
|---|---|---|
| Domain | Business errors | Order cannot be cancelled |
| Application | Use-case errors | Customer not found |
| Infrastructure | Technical errors | Database unavailable |
| API | HTTP errors | 404 Not Found |

Each layer has a different responsibility.

## 4. Domain Errors

Domain errors represent violations of business rules.

They belong to the domain layer.

Examples:

- Invalid order status.
- Product cannot be published.
- Insufficient inventory.
- Refund exceeds the allowed amount.
- Customer account is inactive.

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Order cannot be placed")]
    CannotPlaceOrder,

    #[error("Order cannot be cancelled")]
    CannotCancelOrder,

    #[error("Order has no items")]
    EmptyOrder,
}
```

Domain errors should not depend on HTTP status codes, SQL errors, or external frameworks.

The domain should describe what is wrong in business terms.

## 5. Application Errors

Application errors represent failures while executing a use case.

Examples:

- Order not found.
- Customer not found.
- Payment operation failed.
- Required dependency unavailable.
- Transaction could not be committed.

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Customer not found")]
    CustomerNotFound,

    #[error(transparent)]
    Domain(#[from] OrderError),

    #[error("Operation failed")]
    OperationFailed,
}
```

The application layer translates errors from the domain and infrastructure into errors meaningful to the use case.

Avoid putting HTTP-specific behavior in application errors.

## 6. Infrastructure Errors

Infrastructure errors represent technical failures.

Examples:

- PostgreSQL connection failure.
- SQL constraint violation.
- External API timeout.
- Serialization failure.
- File system error.

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Database operation failed")]
    Database(#[from] sqlx::Error),

    #[error("Record not found")]
    NotFound,
}
```

Infrastructure errors should be translated before reaching the API boundary.

Do not expose raw SQL errors or database details to API consumers.

## 7. API Errors

The API layer translates application errors into HTTP responses.

For `bio-com`, use a consistent JSON response structure.

Example:

```json
{
  "error": {
    "code": "ORDER_NOT_FOUND",
    "message": "The requested order was not found.",
    "correlation_id": "request-123"
  }
}
```

The response should contain:

| Field | Purpose |
|---|---|
| code | Stable application error code |
| message | Safe message for the client |
| correlation_id | Identifier for troubleshooting |
| details | Optional validation details |

Do not return internal stack traces, database queries, or secrets.

## 8. HTTP Status Mapping

Use consistent HTTP status codes.

| HTTP status | Meaning | Example |
|---|---|---|
| 400 | Bad Request | Invalid request structure |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Order does not exist |
| 409 | Conflict | Invalid state transition |
| 422 | Unprocessable Content | Business validation failure |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Unexpected application failure |
| 502 | Bad Gateway | Invalid upstream response |
| 503 | Service Unavailable | Temporary dependency outage |
| 504 | Gateway Timeout | Upstream request timeout |

The mapping should reflect the actual error semantics, not just the technical source of the error.

For example, a database record not found may become HTTP 404, while an unexpected database connection failure should normally become HTTP 503 or 500 depending on the failure.

## 9. Error Mapping

Errors should be translated as they cross architectural boundaries.

```text
Infrastructure Error
       |
       v
Application Error
       |
       v
API Error
       |
       v
HTTP Response
```

Example:

```text
SQLx connection failure
       |
       v
PersistenceError
       |
       v
ApplicationError::DependencyUnavailable
       |
       v
ApiError::ServiceUnavailable
       |
       v
HTTP 503
```

The API should never need to understand SQLx-specific error types.

## 10. Validation Errors

Validation happens at different levels.

### API validation

Checks request structure and basic input constraints.

Examples:
- Missing required fields.
- Invalid UUID.
- Invalid email format.
- Invalid JSON.

### Domain validation

Checks business rules.

Examples:
- An order must contain at least one item.
- A product price cannot be negative.
- A cancelled order cannot be shipped.

### Application validation

Checks use-case requirements.

Examples:
- Customer exists.
- Required permissions are available.
- An operation can proceed with the current application state.

Keep these responsibilities separate.

Example API response:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "The request contains invalid fields.",
    "details": [
      {
        "field": "quantity",
        "message": "Quantity must be greater than zero."
      }
    ]
  }
}
```

Do not rely on API validation alone. Important business rules must remain protected inside the domain.

## 11. Error Codes

Use stable machine-readable error codes.

Examples:

| Error code | Meaning |
|---|---|
| ORDER_NOT_FOUND | Order does not exist |
| ORDER_ALREADY_CANCELLED | Order is already cancelled |
| ORDER_INVALID_STATUS | Operation not allowed for current status |
| PRODUCT_NOT_FOUND | Product does not exist |
| INSUFFICIENT_INVENTORY | Not enough inventory |
| CUSTOMER_NOT_FOUND | Customer does not exist |
| VALIDATION_ERROR | Invalid request data |
| UNAUTHORIZED | Authentication required |
| FORBIDDEN | Access denied |
| INTERNAL_ERROR | Unexpected failure |

Error codes should remain stable even when internal implementation changes.

## 12. Retryable and Non-Retryable Errors

Not every error should be retried.

### Retryable errors

Examples:
- Temporary database connection failure.
- Network timeout.
- HTTP 429.
- HTTP 503.
- Temporary external service failure.

### Non-retryable errors

Examples:
- Invalid request.
- Missing required information.
- Business rule violation.
- Unauthorized operation.
- Unsupported operation.

This distinction is especially important for background jobs and the transactional outbox.

Example:

```rust
pub enum FailureKind {
    Transient,
    Permanent,
}
```

The retry decision should be made according to the operation and its idempotency guarantees.

Do not automatically retry every HTTP 5xx response without considering the operation.

## 13. Error Handling in Rust

Use typed errors instead of returning arbitrary strings.

Recommended libraries:

| Library | Responsibility |
|---|---|
| `thiserror` | Define application and domain errors |
| `anyhow` | Optional error aggregation in executable boundaries |
| `tracing` | Error logging and diagnostics |
| `axum` | HTTP error response integration |

Use `thiserror` for reusable library and application error types.

`anyhow` can be useful at executable boundaries, but should not replace typed domain or application errors.

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Order is empty")]
    EmptyOrder,

    #[error("Order cannot be cancelled")]
    CannotCancel,
}
```

For `bio-com`, typed errors should be the default throughout the core application.

## 14. Centralized API Error Handling

The API layer should have one consistent mechanism for converting application errors into HTTP responses.

Conceptual structure:

```rust
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}
```

Implement the appropriate Axum response conversion.

The API boundary should:
1. Receive the application error.
2. Identify its category.
3. Map it to an HTTP status.
4. Create a safe response body.
5. Attach the correlation ID.
6. Log unexpected failures.

Avoid repeating error conversion logic in every endpoint.

## 15. Logging Errors

Use `tracing` to record technical failures.

Example:

```rust
tracing::error!(
    correlation_id = %correlation_id,
    error = %error,
    "Order operation failed"
);
```

Expected business failures should generally not be logged as ERROR.

For example:
- Order not found: INFO or WARN depending on context.
- Invalid business transition: usually INFO or WARN.
- Unexpected database failure: ERROR.
- Internal application failure: ERROR.

Avoid logging the same error at every architectural layer.

Log it once at the appropriate boundary, preserving useful context.

## 16. Security

Error responses can expose sensitive implementation details if they are not handled correctly.

Follow these rules:

- Never expose SQL statements.
- Never expose database credentials.
- Never expose authentication tokens.
- Never return internal stack traces.
- Avoid revealing whether sensitive accounts exist.
- Sanitize untrusted input before logging.
- Return generic messages for unexpected internal failures.
- Include correlation IDs for support and troubleshooting.

The application should provide enough information to investigate a problem without exposing internal details.

## 17. Suggested Rust Project Structure

```text
src/
├── shared/
│   ├── domain/
│   │   └── errors/
│   │       └── mod.rs
│   │
│   ├── application/
│   │   └── errors/
│   │       └── mod.rs
│   │
│   └── infrastructure/
│       └── errors/
│           └── mod.rs
│
├── api/
│   └── errors/
│       ├── mod.rs
│       └── response.rs
│
└── modules/
    ├── ordering/
    ├── catalog/
    ├── customer/
    └── identity/
```

Each module should also define its own business-specific errors.

Do not create one enormous global error enum containing every possible business failure.

## 18. Testing Strategy

### Domain tests

Verify:
- Invalid business operations return the correct error.
- Business rules cannot be bypassed.
- Error types are predictable.

### Application tests

Verify:
- Repository errors are translated correctly.
- Missing resources produce the expected application errors.
- Use cases preserve meaningful error information.

### API tests

Verify:
- Correct HTTP status codes.
- Correct error codes.
- Consistent JSON response structure.
- Correlation IDs are included.
- Internal details are not exposed.

### Integration tests

Verify:
- Database failures are handled safely.
- Transaction failures result in rollback.
- Background jobs classify failures correctly.
- Retryable errors are retried according to policy.

## 19. What We Will Not Implement Now

To keep the design simple:

- No complex global exception framework.
- No arbitrary string-based errors.
- No HTTP dependencies in the domain layer.
- No database-specific errors in API responses.
- No automatic retries for all errors.
- No exposure of internal stack traces.

We will use Rust's typed error system and explicit mapping.

## 20. Future Improvements

As the application grows, we can introduce:

- Centralized error tracking.
- Standardized problem-details responses.
- More detailed validation metadata.
- Advanced error classification.
- Automated incident reporting.
- Error-rate dashboards.

These improvements should not complicate the core domain model.

## 21. Summary

For `bio-com`, error handling follows these principles:

- Typed Rust errors.
- Domain-specific business errors.
- Application-level error translation.
- Infrastructure error isolation.
- Centralized API error mapping.
- Stable machine-readable error codes.
- Consistent HTTP status codes.
- Retryable and permanent failure classification.
- Structured error logging.
- No sensitive information in API responses.

The objective is to make failures predictable, understandable, and safe across the entire application.
