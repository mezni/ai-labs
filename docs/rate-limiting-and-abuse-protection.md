# Rate Limiting and Abuse Protection

## 1. Purpose

Rate limiting controls how often a client can call the API within a given period.

It protects the application from:

- Too many requests from one client.
- Brute-force login attempts.
- Automated checkout abuse.
- Repeated payment requests.
- Expensive or unnecessary API calls.
- Accidental traffic spikes.

The goal is to keep the platform available and protect important business operations without making normal usage difficult.

## 2. Design Principles

The initial implementation follows these principles:

- Apply limits at the API boundary.
- Use different limits for different operations.
- Identify clients using authenticated users or IP addresses.
- Protect sensitive operations more strictly.
- Return a clear response when a limit is reached.
- Avoid introducing Redis or another infrastructure dependency initially.
- Keep rate-limiting logic separate from business logic.

Rate limiting is a technical protection mechanism. It must not replace business rules such as stock availability, payment validation, or order eligibility.

## 3. Where Rate Limiting Applies

Not every endpoint needs the same limit.

For example, browsing products is different from attempting to log in or submitting a payment.

| API operation | Protection level | Reason |
|---|---|---|
| Browse products | Normal | Prevent excessive browsing traffic |
| Search products | Normal | Search may require more database work |
| View product details | Normal | Protect API availability |
| Register account | Strict | Prevent automated account creation |
| Login | Very strict | Prevent brute-force attacks |
| Password reset | Very strict | Prevent account abuse |
| Add item to cart | Normal | Prevent excessive requests |
| Checkout | Strict | Protect order creation |
| Create payment | Very strict | Prevent duplicate or abusive payment attempts |
| Admin operations | Strict | Protect privileged functionality |

The actual limits should be configurable and adjusted using production metrics.

## 4. Rate-Limiting Strategies

### 4.1 IP-Based Limits

IP-based limits are useful for anonymous users and public endpoints.

Example:

- Allow a limited number of login attempts from one IP address.
- Limit account registration requests.
- Limit password reset requests.

However, IP addresses are not reliable user identities.

Several legitimate users may share one public IP address. A user may also change networks or use a VPN.

Therefore, IP-based limits should not be the only protection for authenticated users.

### 4.2 User-Based Limits

For authenticated requests, use the authenticated user's ID.

Example:

- Limit checkout attempts per user.
- Limit cart modifications.
- Limit sensitive account operations.

The user ID must come from the authenticated identity, not from a value supplied in the request body.

### 4.3 Endpoint-Based Limits

Some endpoints need stricter protection than others.

For example, product browsing can have a higher limit than password reset.

The rate limiter should support policies based on:

- HTTP method.
- API route.
- Authenticated user.
- Client IP address.
- Operation type.

Avoid creating a separate rate-limiting implementation for every endpoint. Use reusable policies.

## 5. Initial Rate-Limit Policies

The following values are starting examples, not final production limits.

| Operation | Example limit | Window |
|---|---:|---|
| Product browsing | 120 requests | 1 minute |
| Product search | 30 requests | 1 minute |
| Login | 5 attempts | 1 minute |
| Password reset | 3 requests | 15 minutes |
| Account registration | 5 requests | 1 hour |
| Checkout | 5 attempts | 10 minutes |
| Payment creation | 5 attempts | 10 minutes |

These limits should be configurable through application configuration.

For login and password reset, combine IP-based and account-based protections where appropriate.

Do not reveal whether an email address belongs to an existing account when enforcing these limits.

## 6. Rate-Limiting Algorithm

For the first version, use a fixed-window counter.

The application tracks the number of requests for a given client and operation during a time window.

Example:

A user can make 10 requests per minute.

- The first request starts the window.
- Each request increments the counter.
- Requests are accepted until the limit is reached.
- Further requests are rejected until the window expires.

The fixed-window algorithm is simple to implement and understand.

Its main limitation is the boundary effect: a client may make requests at the end of one window and immediately make more requests at the beginning of the next.

This is acceptable for the initial implementation.

A sliding-window or token-bucket algorithm can be introduced later if production traffic requires smoother limits.

## 7. Initial Architecture

Rate limiting belongs in the API layer, before application use cases are executed.

```text
                 Client
                   |
                   v
             Axum Router
                   |
                   v
            Authentication
                   |
                   v
             Rate Limiter
                   |
             +-----+-----+
             |           |
          Allowed      Rejected
             |           |
             v           v
       Application    HTTP 429
          Layer
             |
             v
         Domain
             |
             v
       Infrastructure
```

The rate limiter should not depend on domain aggregates or business rules.

It only decides whether a request can continue.

## 8. Rust Implementation Design

Create a reusable rate-limiting component.

Suggested structure:

```text
src/
├── infrastructure/
│   └── rate_limiting/
│       ├── mod.rs
│       ├── policy.rs
│       ├── limiter.rs
│       ├── memory_store.rs
│       └── postgres_store.rs
│
├── interfaces/
│   └── http/
│       ├── middleware/
│       │   └── rate_limit.rs
│       └── routes/
│
└── application/
```

Responsibilities:

| Component | Responsibility |
|---|---|
| `policy.rs` | Defines limits and time windows |
| `limiter.rs` | Applies rate-limit rules |
| `memory_store.rs` | Stores counters in process memory |
| `postgres_store.rs` | Stores shared counters in PostgreSQL |
| `rate_limit.rs` | Connects rate limiting to Axum middleware |

The rate limiter should expose a simple interface.

Example:

```rust
pub struct RateLimitPolicy {
    pub max_requests: u32,
    pub window_seconds: u64,
}

pub enum RateLimitDecision {
    Allowed {
        remaining: u32,
        reset_after_seconds: u64,
    },
    Rejected {
        retry_after_seconds: u64,
    },
}
```

The HTTP middleware should not know how counters are stored. It should depend on an abstraction implemented by the selected storage mechanism.

## 9. Storage Strategy

### 9.1 In-Memory Storage

For local development and a single application instance, an in-memory counter is sufficient.

Advantages:

- Simple.
- Fast.
- No additional infrastructure.
- Easy to test.

Limitations:

- Counters disappear when the application restarts.
- Multiple application instances have independent counters.
- It cannot enforce a shared global limit.

This is a reasonable starting point for development, but it should not be treated as distributed rate limiting.

### 9.2 PostgreSQL Storage

When shared limits are required, use PostgreSQL.

Example table:

```sql
CREATE TABLE rate_limit_counters (
    id UUID PRIMARY KEY,
    rate_limit_key VARCHAR(255) NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT uq_rate_limit_window
        UNIQUE (rate_limit_key, window_start)
);

CREATE INDEX idx_rate_limit_counters_expires_at
    ON rate_limit_counters (expires_at);
```

The `rate_limit_key` identifies the client and operation.

Example keys:

```text
ip:192.0.2.10:login
user:123:checkout
user:123:payment-create
ip:192.0.2.10:registration
```

Use an atomic database operation to increment counters. Do not read a counter, increment it in application memory, and write it back: concurrent requests could exceed the configured limit.

PostgreSQL can coordinate counters across multiple application instances, but this adds database traffic. Monitor its impact before using it for every public request.

For high-volume traffic, a dedicated distributed rate-limiting store may eventually be more appropriate.

## 10. HTTP Response

When a client exceeds a limit, return:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 30
Content-Type: application/json
```

Example response:

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please try again later."
  }
}
```

The `Retry-After` header tells the client when it can retry.

Where useful, also return standard rate-limit headers describing the limit, remaining requests, and reset time.

Do not expose internal counter keys, database details, or security rules.

## 11. Abuse Protection for Sensitive Operations

Rate limiting alone is not enough to protect the application.

### 11.1 Login Protection

Apply several controls:

- Limit login attempts per IP.
- Limit repeated attempts against an account identifier.
- Use secure password hashing.
- Return generic authentication errors.
- Avoid revealing whether an account exists.
- Record suspicious login activity.

Do not permanently lock an account just because someone repeatedly attempts to log in. An attacker could use this to deny service to legitimate users.

### 11.2 Registration Protection

Apply:

- IP-based limits.
- Account creation limits.
- Email verification where required.
- Input validation.
- Duplicate account checks.

A CAPTCHA or similar challenge can be added later if automated registration becomes a real problem.

### 11.3 Checkout Protection

Protect checkout against:

- Repeated order creation.
- Automated requests.
- Invalid cart submissions.
- Repeated attempts with the same cart.

Use idempotency controls where appropriate so that retrying a request does not create duplicate orders.

Rate limiting does not replace order validation or inventory reservation.

### 11.4 Payment Protection

Payment operations need additional safeguards:

- Validate the order before creating a payment.
- Use idempotency keys for payment creation.
- Prevent duplicate payment processing.
- Validate payment provider callbacks.
- Record payment attempts for audit purposes.

A rate limit must never be the only mechanism preventing duplicate charges.

## 12. Request Size and Timeout Protection

Abuse can also happen through large or slow requests.

Configure limits for:

- Maximum request body size.
- Maximum header size.
- Request processing time.
- Database query time.
- External payment provider timeouts.

For example:

- Reject unexpectedly large JSON payloads.
- Set reasonable timeouts for external services.
- Avoid allowing clients to hold connections indefinitely.

These settings should be centralized in the HTTP server configuration.

## 13. Trusted Proxy and Client IP

When the application runs behind a reverse proxy, the network connection may come from the proxy rather than the original client.

The application may need to read forwarded IP headers.

However, headers such as `X-Forwarded-For` can be spoofed if they are accepted from arbitrary clients.

Only trust forwarded headers when they are added by known, trusted proxies.

The trusted proxy configuration must be explicit.

Never use an untrusted forwarded IP header as the only basis for security decisions.

## 14. Configuration

Rate limits should be configurable without changing business logic.

Example:

```toml
[rate_limiting]
enabled = true
storage = "memory"

[rate_limiting.product_browsing]
max_requests = 120
window_seconds = 60

[rate_limiting.login]
max_requests = 5
window_seconds = 60

[rate_limiting.password_reset]
max_requests = 3
window_seconds = 900

[rate_limiting.checkout]
max_requests = 5
window_seconds = 600
```

In production, sensitive settings should be validated at startup.

Do not allow accidental configuration values that disable important protections.

## 15. Logging and Monitoring

Record useful information about rate-limit activity:

- Endpoint.
- Operation.
- Limit type.
- Rejected request count.
- Authenticated user ID when available.
- Client IP when appropriate.
- Timestamp.

Monitor:

- Number of rejected requests.
- Most frequently limited endpoints.
- Repeated login failures.
- Checkout abuse patterns.
- PostgreSQL counter storage performance.

Do not log passwords, authentication tokens, payment credentials, or unnecessary personal information.

Avoid using user IDs or IP addresses as metric labels because they create high-cardinality metrics. Use structured logs for detailed investigation instead.

## 16. Testing Strategy

Test the rate limiter independently from the HTTP application.

Important test cases:

1. Requests below the limit are accepted.
2. The request reaching the limit is accepted.
3. The next request is rejected.
4. Requests are accepted after the window expires.
5. Different users have independent limits.
6. Different endpoints have independent policies.
7. Anonymous requests use the correct IP-based identity.
8. Concurrent requests do not exceed the limit when using shared storage.
9. Expired counters can be cleaned up.
10. Disabled rate limiting behaves as configured.
11. Untrusted forwarded headers are ignored.
12. Rejected requests return HTTP 429 and `Retry-After`.

Use an injectable clock for deterministic window and expiration tests.

## 17. Implementation Plan

Implement this feature gradually.

| Step | Work |
|---|---|
| 1 | Define rate-limit policies |
| 2 | Implement the rate-limiter interface |
| 3 | Add in-memory counter storage |
| 4 | Add Axum middleware |
| 5 | Protect login and registration |
| 6 | Protect checkout and payment endpoints |
| 7 | Add request-size and timeout limits |
| 8 | Add structured logging and metrics |
| 9 | Add PostgreSQL storage when multiple instances require shared limits |
| 10 | Run concurrency and integration tests |

Start with a small set of sensitive endpoints. Expand coverage based on real usage and monitoring.

## 18. Future Improvements

The architecture should allow future improvements without changing domain logic:

- Token-bucket rate limiting.
- Sliding-window counters.
- Distributed rate limiting.
- Adaptive limits based on suspicious activity.
- CAPTCHA integration.
- Dedicated bot detection.
- Centralized edge or reverse-proxy rate limiting.

These are future options, not requirements for the first release.

## 19. Final Decision

For the initial `bio-com` release:

- Use Axum middleware for API rate limiting.
- Use configurable policies per endpoint.
- Apply IP-based limits to anonymous operations.
- Apply user-based limits to authenticated operations.
- Use in-memory storage for local development and a single-instance starting point.
- Introduce PostgreSQL-backed counters when shared limits are needed.
- Protect login, registration, checkout, and payment operations more strictly.
- Return HTTP 429 with a `Retry-After` header.
- Keep rate limiting outside the domain layer.
- Do not introduce Redis, Kafka, or another infrastructure component just for rate limiting.

The design keeps the application simple while leaving room to scale when actual traffic requires it.
