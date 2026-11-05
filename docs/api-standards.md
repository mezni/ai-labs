# API Standards

## 1. Purpose

This document defines the API standards used across the bio-com platform.

Every module must follow the same conventions when exposing HTTP endpoints.

These standards cover:

- URL naming.
- HTTP methods.
- Request and response formats.
- Status codes.
- Error handling.
- Pagination.
- Filtering and sorting.
- API versioning.
- Validation.
- Authentication and authorization.

The API layer uses Rust and Axum.

## 2. General Principles

Our API follows these principles:

- Use REST conventions.
- Use JSON for request and response bodies.
- Use resource-oriented URLs.
- Use standard HTTP methods and status codes.
- Keep request and response formats consistent.
- Validate input before executing business operations.
- Never expose internal domain models directly.
- Keep API contracts independent of database structures.

## 3. Base URL and Versioning

All API endpoints use a versioned base path.

```text
/api/v1
```

Examples:

```text
/api/v1/products
/api/v1/categories
/api/v1/orders
/api/v1/customers
/api/v1/auth/login
```

### Versioning Rules

- Use URL-based versioning.
- Introduce a new version when making breaking changes.
- Keep existing versions operational during a reasonable migration period.
- Document deprecated endpoints.
- Avoid introducing a new version for backward-compatible changes.

## 4. URL Naming Conventions

Use plural nouns for collections.

| Good | Avoid |
|---|---|
| /products | /getProducts |
| /orders | /getOrders |
| /customers | /customer |
| /categories | /categoryList |

Use lowercase words separated by hyphens when necessary.

Example:

```text
/api/v1/shipping-addresses
/api/v1/order-items
```

### Resource Relationships

Use nested URLs only when the relationship is clear.

Example:

```text
/api/v1/orders/{order_id}/items
```

Avoid deeply nested URLs.

## 5. HTTP Methods

Use HTTP methods according to their intended meaning.

| Method | Purpose | Example |
|---|---|---|
| GET | Retrieve resources | GET /products |
| POST | Create a resource or execute an operation | POST /orders |
| PUT | Replace a resource | PUT /products/{id} |
| PATCH | Partially update a resource | PATCH /products/{id} |
| DELETE | Delete a resource | DELETE /products/{id} |

### Important Rules

GET requests must not change business state.

PUT should represent a complete replacement of the relevant resource representation.

PATCH should update only the supplied fields.

DELETE should be used only when deletion is permitted by the domain rules.

For business operations such as cancellation and refunds, explicit action endpoints are acceptable.

Examples:

```text
POST /api/v1/orders/{id}/cancel
POST /api/v1/payments/{id}/refund
```

These operations must still enforce authorization and domain rules.

## 6. Request Standards

Use JSON for request bodies.

Example:

```json
{
  "name": "Wireless Headphones",
  "description": "Bluetooth headphones",
  "price": 89.99,
  "currency": "CAD"
}
```

### Request Rules

- Use `snake_case` for JSON fields.
- Validate required fields.
- Reject invalid data types.
- Apply sensible maximum lengths.
- Validate identifiers.
- Never trust values supplied by the client for authorization.
- Do not accept internal database fields unless explicitly part of the API contract.

For example, the server should determine the authenticated customer ID rather than trusting a customer ID supplied in an order creation request.

## 7. Response Standards

Use consistent JSON responses.

### Successful Response

Example:

```json
{
  "data": {
    "id": "product-123",
    "name": "Wireless Headphones",
    "price": 89.99,
    "currency": "CAD"
  }
}
```

### Collection Response

Example:

```json
{
  "data": [
    {
      "id": "product-123",
      "name": "Wireless Headphones",
      "price": 89.99,
      "currency": "CAD"
    }
  ],
  "meta": {
    "page": 1,
    "page_size": 20,
    "total_items": 45,
    "total_pages": 3
  }
}
```

### Empty Response

For operations that do not need a response body, use:

```text
204 No Content
```

Do not return an unnecessary JSON body with a 204 response.

## 8. Error Response Standards

All API errors must use a consistent structure.

Example:

```json
{
  "error": {
    "code": "PRODUCT_NOT_FOUND",
    "message": "The requested product was not found.",
    "details": [],
    "correlation_id": "request-123"
  }
}
```

### Error Fields

| Field | Description |
|---|---|
| code | Stable application error code |
| message | Human-readable message |
| details | Optional validation or error details |
| correlation_id | Identifier used to trace the request |

Error codes should remain stable even if the message changes.

### Validation Error

Example:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "The request contains invalid fields.",
    "details": [
      {
        "field": "price",
        "code": "MUST_BE_POSITIVE",
        "message": "Price must be greater than zero."
      }
    ],
    "correlation_id": "request-456"
  }
}
```

## 9. HTTP Status Codes

Use standard HTTP status codes consistently.

| Status | Meaning | Example |
|---|---|---|
| 200 | OK | Resource retrieved |
| 201 | Created | Product created |
| 202 | Accepted | Asynchronous operation accepted |
| 204 | No Content | Resource deleted |
| 400 | Bad Request | Invalid request |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Duplicate or conflicting operation |
| 422 | Unprocessable Content | Business rule violation |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Unexpected failure |
| 503 | Service Unavailable | Temporary service unavailability |

Use 401 for authentication failures and 403 for authorization failures.

Do not expose internal errors to API consumers.

## 10. Pagination

Collection endpoints must support pagination when results can grow significantly.

Use page-based pagination initially.

Example:

```text
GET /api/v1/products?page=1&page_size=20
```

### Parameters

| Parameter | Description | Default |
|---|---|---|
| page | Requested page | 1 |
| page_size | Number of records | 20 |

Set a maximum page size, initially 100.

### Response

```json
{
  "data": [],
  "meta": {
    "page": 1,
    "page_size": 20,
    "total_items": 120,
    "total_pages": 6
  }
}
```

Use deterministic sorting to avoid inconsistent pagination results.

For very large datasets, cursor-based pagination can be introduced later.

## 11. Filtering and Sorting

Collection endpoints may support filtering and sorting.

Example:

```text
GET /api/v1/products?category_id=cat-123&status=ACTIVE
```

Sorting example:

```text
GET /api/v1/products?sort_by=created_at&sort_order=desc
```

Rules:

- Only allow documented filter fields.
- Only allow documented sorting fields.
- Validate sorting direction.
- Never use raw client input directly in SQL expressions.
- Apply pagination after filtering and sorting.

## 12. Authentication and Authorization

Protected endpoints require a valid authenticated session.

The initial implementation uses secure session cookies.

Example:

```text
Cookie: bio_com_session=<session-token>
```

The token must be transmitted only over HTTPS in production.

### Authorization Rules

Each protected operation must check:

1. User authentication.
2. Required permission.
3. Resource ownership when applicable.

Examples:

| Endpoint | Required Permission |
|---|---|
| GET /products | product:read |
| POST /products | product:create |
| PATCH /products/{id} | product:update |
| GET /orders | order:read_own |
| GET /admin/orders | order:read_all |

Authorization must be enforced in the backend.

## 13. Idempotency

Some operations must prevent duplicate execution.

This is particularly important for:

- Order creation.
- Payment initiation.
- Refund requests.
- Checkout operations.

For supported operations, clients can send:

```text
Idempotency-Key: unique-client-generated-key
```

The server must associate the key with the authenticated actor, operation, and request.

Repeated requests with the same key and equivalent payload must return the original result rather than execute the operation again.

Reusing a key with a different payload must return a conflict.

Persist idempotency records in PostgreSQL.

Idempotency does not replace database constraints or domain-level duplicate checks.

## 14. API Validation

Validation happens at multiple levels.

### API Layer

Checks:

- Required fields.
- Data formats.
- Request size.
- Identifier formats.
- Numeric ranges.

### Application Layer

Checks:

- Use case requirements.
- Authorization.
- Application workflows.

### Domain Layer

Protects business invariants.

For example:

A product price cannot be negative.

An order cannot be cancelled after it has reached a state where cancellation is prohibited.

A refund cannot exceed the refundable amount.

Do not rely exclusively on API validation to protect business rules.

## 15. Date and Time Standards

Use ISO 8601 timestamps in UTC.

Example:

```text
2026-11-05T13:45:00Z
```

Rules:

- Use UTC for persisted timestamps.
- Return timestamps in ISO 8601 format.
- Do not use local server time as a business rule.
- Handle customer time zones explicitly when displaying dates.

For dates without a time component, use:

```text
2026-11-05
```

## 16. Monetary Values

Money must be represented without floating-point arithmetic.

For API contracts, use decimal strings to avoid precision loss.

Example:

```json
{
  "amount": "89.99",
  "currency": "CAD"
}
```

Rules:

- Always include currency.
- Never use floating-point types for financial calculations.
- Validate currency codes.
- Use the appropriate currency precision.
- Perform monetary calculations using a decimal or integer minor-unit representation.

The domain layer remains responsible for monetary correctness.

## 17. API Documentation

All public API endpoints must be documented.

Use OpenAPI to describe:

- Endpoint paths.
- HTTP methods.
- Request schemas.
- Response schemas.
- Authentication requirements.
- Error responses.
- Query parameters.

Documentation should reflect the actual API contract.

Update the OpenAPI specification whenever an API contract changes.

## 18. API Testing

Each module must test its API contract.

### Functional Tests

- Successful requests.
- Invalid requests.
- Missing resources.
- Business rule violations.
- Correct status codes.

### Security Tests

- Missing authentication.
- Invalid sessions.
- Insufficient permissions.
- Resource ownership violations.
- Invalid request payloads.

### Contract Tests

- Response structure.
- Field types.
- Required fields.
- Error format.
- OpenAPI consistency.

## 19. Implementation Guidelines

The API layer uses:

| Responsibility | Technology |
|---|---|
| HTTP server | Axum |
| Async runtime | Tokio |
| Serialization | Serde |
| Validation | Application/API validation |
| Database | PostgreSQL |
| Database access | SQLx |
| API documentation | OpenAPI |

Keep HTTP handlers thin.

A handler should:

1. Extract and validate the request.
2. Obtain the authenticated identity.
3. Call the appropriate application use case.
4. Convert the result into an HTTP response.

Business logic must remain outside the HTTP handler.

## 20. Key Design Decisions

1. REST-style APIs with JSON.
2. URL-based versioning using `/api/v1`.
3. `snake_case` for JSON fields.
4. Consistent success and error response structures.
5. Page-based pagination initially.
6. Explicit filtering and sorting.
7. Secure cookie-based authentication.
8. Backend authorization and resource ownership checks.
9. PostgreSQL-backed idempotency for sensitive operations.
10. ISO 8601 timestamps in UTC.
11. Decimal strings for monetary API values.
12. OpenAPI documentation for API contracts.
13. Thin Axum handlers with business logic in application use cases.

---

**Main principle:** Every bio-com module must expose APIs that follow the same conventions, regardless of its internal domain logic.
