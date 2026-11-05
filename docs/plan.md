# Build Plan

## 1. Purpose

This document defines the implementation roadmap for the `bio-com` e-commerce platform.

The objective is to build a production-ready modular monolith using Rust, PostgreSQL, and React.

Implementation follows an incremental approach. Each stage delivers working functionality, automated tests, and updated documentation.

We will prioritize correctness, maintainability, security, and simplicity over introducing unnecessary infrastructure.

## 2. Technology Stack

| Area | Technology |
|---|---|
| Backend | Rust |
| Frontend | TypeScript / React |
| Frontend tooling | Vite |
| Database | PostgreSQL |
| Database access | SQLx |
| API | REST / JSON |
| API documentation | OpenAPI |
| Authentication | Secure session-based authentication |
| Authorization | RBAC and permissions |
| Data generation | Rust |
| Backend testing | Cargo test |
| Frontend testing | Vitest |
| E2E testing | Playwright |
| Logging | Rust tracing |
| Configuration | Environment variables and TOML |
| Deployment | Lightweight container or server deployment |

## 3. Implementation Strategy

We will follow this sequence:

1. Repository and workspace foundation.
2. Technical architecture foundation.
3. Database foundation.
4. Identity and access management.
5. Product catalog.
6. Inventory management.
7. Customer management.
8. Shopping cart.
9. Order management.
10. Payment management.
11. Shipping and fulfillment.
12. Refund management.
13. Background jobs and domain events.
14. Data generator.
15. REST API completion.
16. React web application.
17. Integration and end-to-end testing.
18. Security and production readiness.

The implementation is incremental. We will create working vertical slices rather than completing every database table before implementing application behavior.

## 4. Build Stages

### Stage 1 — Repository and Workspace Foundation

**Goal:** Establish the project structure and development workflow.

Tasks:

- Create the `bio-com` repository.
- Configure the Rust workspace.
- Create the backend application.
- Create the React application.
- Create the shared Rust crates.
- Configure formatting and linting.
- Configure environment variables.
- Add `.gitignore`.
- Add README.md.
- Add initial CI workflow.

Expected structure:

```text
bio-com/
├── apps/
│   ├── api/
│   └── web/
│
├── crates/
│   ├── domain/
│   ├── application/
│   ├── infrastructure/
│   └── shared/
│
├── tools/
│   └── data-generator/
│
├── migrations/
├── docs/
├── tests/
├── scripts/
├── Cargo.toml
└── README.md
```

**Exit criteria:**

- Rust workspace builds.
- React application starts.
- Formatting and linting work.
- CI executes successfully.

Suggested commit:

```bash
git commit -m "chore(bio-com): initialize project workspace"
```

### Stage 2 — Technical Architecture Foundation

**Goal:** Establish Clean Architecture and modular boundaries.

Tasks:

- Define domain crate responsibilities.
- Define application crate responsibilities.
- Define infrastructure crate responsibilities.
- Define API application responsibilities.
- Establish dependency direction.
- Define module boundaries.
- Establish error-handling conventions.
- Define configuration management.
- Define logging conventions.
- Define dependency injection strategy.

Expected dependency direction:

```text
       API
        |
        v
   Application
        |
        v
      Domain

Infrastructure
       |
       +---- Implements application/domain ports
```

The domain must not depend on infrastructure.

The application layer defines use cases and ports.

Infrastructure implements database access and external integrations.

**Exit criteria:**

- Workspace boundaries are documented.
- Dependency direction is enforced where practical.
- A sample use case works.
- Unit tests demonstrate the architecture.

Suggested commit:

```bash
git commit -m "feat(bio-com): establish clean architecture foundation"
```

### Stage 3 — Database Foundation

**Goal:** Establish reliable PostgreSQL persistence.

Tasks:

- Configure SQLx.
- Configure connection pooling.
- Implement database configuration.
- Create migration structure.
- Define common database conventions.
- Configure transactions.
- Define UUID strategy.
- Define timestamp conventions.
- Define monetary precision.
- Configure migration execution.
- Add database health checks.

Start with shared persistence requirements before creating business tables.

**Exit criteria:**

- PostgreSQL connection works.
- Migrations execute successfully.
- Database transactions work.
- Connection failures are handled.
- Integration tests use a dedicated database.

Suggested commit:

```bash
git commit -m "feat(bio-com): establish PostgreSQL persistence"
```

### Stage 4 — Identity and Access Management

**Goal:** Build the authentication and authorization foundation.

Identity and Access Management is a dedicated business module inside the modular monolith.

It is not a separately deployed microservice.

Tasks:

- User registration.
- Login.
- Logout.
- Password hashing.
- Session management.
- Session expiration.
- Password change.
- Password reset workflow.
- Role management.
- Permission management.
- User-role assignment.
- Authorization policies.
- Audit events.
- Authentication middleware.

Initial roles:

- CUSTOMER
- ADMIN
- INVENTORY_MANAGER
- SUPPORT_AGENT

Use permissions for fine-grained access control.

For example:

```text
PRODUCT_READ
PRODUCT_CREATE
PRODUCT_UPDATE
INVENTORY_READ
INVENTORY_ADJUST
ORDER_READ
ORDER_UPDATE
REFUND_APPROVE
CUSTOMER_READ
```

Security requirements:

- Never store plaintext passwords.
- Use a suitable password hashing algorithm such as Argon2id.
- Use secure HTTP-only cookies.
- Apply CSRF protection where applicable.
- Enforce authorization on the backend.
- Protect sensitive operations with audit logging.

**Exit criteria:**

- Registration and login work.
- Sessions are created and invalidated correctly.
- Permissions are enforced.
- Unauthorized access is rejected.
- Security tests pass.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement identity and access management"
```

### Stage 5 — Product Catalog

**Goal:** Build product discovery and catalog management.

Tasks:

- Product categories.
- Products.
- Product variants.
- SKU management.
- Product descriptions.
- Product pricing.
- Product status.
- Product search.
- Product filtering.
- Product pagination.
- Product administration.

Start with catalog domain modeling and business rules.

Then implement application use cases, persistence, and API endpoints.

**Exit criteria:**

- Products can be created and updated.
- Product variants are supported.
- SKUs are unique.
- Product listing and filtering work.
- Inactive products are handled correctly.
- Domain and integration tests pass.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement product catalog"
```

### Stage 6 — Inventory Management

**Goal:** Manage stock and inventory movements.

Tasks:

- Inventory locations.
- Stock quantities.
- Available quantities.
- Reserved quantities.
- Stock adjustments.
- Stock movements.
- Inventory reservation.
- Inventory release.
- Stock validation.

Important business rule:

Available stock must account for reserved stock.

Inventory operations must be transactionally consistent.

Prevent concurrent requests from overselling available inventory.

**Exit criteria:**

- Inventory can be created and adjusted.
- Stock reservation works.
- Reservation release works.
- Concurrent updates are safe.
- Inventory movements are auditable.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement inventory management"
```

### Stage 7 — Customer Management

**Goal:** Manage customer profiles and addresses.

Tasks:

- Customer profiles.
- Customer contact information.
- Billing addresses.
- Shipping addresses.
- Address validation.
- Customer status.
- Customer profile updates.
- Customer lookup.

Identity and customer profile responsibilities must remain clearly separated.

Authentication accounts belong to Identity and Access Management.

Commercial customer information belongs to Customer Management.

**Exit criteria:**

- Customers can manage their profiles.
- Customers can manage addresses.
- Customer data is protected.
- Access to customer information is authorized.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement customer management"
```

### Stage 8 — Shopping Cart

**Goal:** Allow customers to prepare purchases.

Tasks:

- Create shopping cart.
- Add cart items.
- Remove cart items.
- Update quantities.
- Calculate cart summary.
- Validate product availability.
- Handle inactive products.
- Handle price changes.

The cart is not the authoritative source of order pricing.

Prices and product availability must be revalidated during checkout.

**Exit criteria:**

- Customers can manage carts.
- Cart items reference valid products.
- Invalid quantities are rejected.
- Cart totals are consistent.
- Cart persistence works.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement shopping cart"
```

### Stage 9 — Order Management

**Goal:** Implement the order lifecycle.

Tasks:

- Create order.
- Create order items.
- Capture product snapshots.
- Calculate order totals.
- Store billing and shipping snapshots.
- Confirm orders.
- Cancel orders.
- Manage order status.
- Retrieve order history.
- Implement order transitions.

Order creation must be transactional.

The order should preserve the product description, price, and other relevant information at purchase time.

Do not rely on the current catalog values to reconstruct historical orders.

**Exit criteria:**

- Orders can be created.
- Order items are preserved.
- Order totals are correct.
- Invalid transitions are rejected.
- Inventory reservation is coordinated correctly.
- Duplicate order submissions are handled safely.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement order management"
```

### Stage 10 — Payment Management

**Goal:** Implement payment orchestration.

Tasks:

- Payment creation.
- Payment status management.
- Payment provider integration boundary.
- Payment authorization.
- Payment capture.
- Payment failure handling.
- Payment reconciliation.
- Idempotency.
- Payment transaction history.

Use a payment provider abstraction.

Do not couple the domain model to one payment provider.

Start with a mock payment provider for local development and testing.

Never store raw card details.

**Exit criteria:**

- Payment lifecycle works.
- Payment failures are handled.
- Duplicate requests do not create duplicate payments.
- Payment state is auditable.
- Provider integration is isolated.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement payment management"
```

### Stage 11 — Shipping and Fulfillment

**Goal:** Implement order fulfillment and shipment tracking.

Tasks:

- Shipment creation.
- Shipment items.
- Shipping methods.
- Shipment status.
- Tracking references.
- Shipment updates.
- Delivery confirmation.
- Failed delivery handling.

Use an external shipping provider abstraction when integration becomes necessary.

Start with a mock shipping implementation.

**Exit criteria:**

- Shipments can be created.
- Shipment items match order items.
- Shipment status transitions work.
- Tracking information is available.
- Delivery status is reflected correctly.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement shipping and fulfillment"
```

### Stage 12 — Refund Management

**Goal:** Manage full and partial refunds.

Tasks:

- Refund requests.
- Refund eligibility.
- Refund reasons.
- Refund approval.
- Refund rejection.
- Partial refunds.
- Full refunds.
- Payment provider refund integration.
- Refund history.
- Audit logging.

Refund eligibility and limits must be enforced by the backend.

Refund processing must account for previous refunds to prevent exceeding the refundable balance.

**Exit criteria:**

- Refund requests work.
- Full and partial refunds work.
- Refund limits are enforced.
- Duplicate refund requests are handled safely.
- Refund status is auditable.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement refund management"
```

### Stage 13 — Domain Events and Background Processing

**Goal:** Support reliable asynchronous business operations.

Tasks:

- Define domain events.
- Define application event handlers.
- Implement event dispatching.
- Implement background workers.
- Implement retry policies.
- Implement failed-job handling.
- Implement scheduling.
- Implement idempotent handlers.

Start with in-process event dispatching and a PostgreSQL-backed job mechanism where durable background execution is required.

Avoid Kafka and external message brokers initially.

Important distinction:

Domain events represent something that happened within the domain.

Application events and handlers coordinate application-level work.

Background jobs execute work asynchronously.

**Exit criteria:**

- Events are dispatched correctly.
- Background jobs survive application restarts where durability is required.
- Retry policies work.
- Duplicate processing is safe.
- Failures are observable.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement domain events and background jobs"
```

### Stage 14 — Data Generator

**Goal:** Provide realistic data for development and testing.

Tasks:

- Implement Rust CLI.
- Implement configuration profiles.
- Generate customers.
- Generate products.
- Generate inventory.
- Generate carts.
- Generate orders.
- Generate payments.
- Generate refunds.
- Generate shipments.
- Implement deterministic generation.
- Implement integrity validation.
- Implement safe reset operations.

Dataset profiles:

| Profile | Customers | Products | Orders |
|---|---:|---:|---:|
| Minimal | 10 | 10 | 10 |
| Small | 100 | 50 | 250 |
| Medium | 1,000 | 500 | 2,500 |
| Large | 10,000 | 2,000 | 25,000 |

These are initial defaults, not fixed limits.

**Exit criteria:**

- All generators work.
- Relationships are valid.
- Seeded generation is reproducible.
- Database integrity checks pass.
- Production database protection works.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement Rust data generator"
```

### Stage 15 — REST API Completion

**Goal:** Expose the business capabilities through consistent HTTP APIs.

Tasks:

- Define endpoint conventions.
- Implement request validation.
- Implement response DTOs.
- Implement pagination.
- Implement filtering.
- Implement structured errors.
- Implement API versioning.
- Generate OpenAPI documentation.
- Implement authentication middleware.
- Implement authorization middleware.
- Add rate limiting.
- Add request correlation IDs.

API endpoints should expose application use cases rather than database operations.

**Exit criteria:**

- API contracts are documented.
- OpenAPI specification is generated.
- Authentication and authorization work.
- API error responses are consistent.
- Rate limiting is tested.
- Integration tests pass.

Suggested commit:

```bash
git commit -m "feat(bio-com): complete REST API foundation"
```

### Stage 16 — React Web Application

**Goal:** Deliver the customer and administration interfaces.

Implement features in this order:

**Customer application**

1. Application layout and navigation.
2. Registration and login.
3. Product catalog.
4. Product details.
5. Shopping cart.
6. Checkout.
7. Order history.
8. Order details.
9. Shipment tracking.
10. Customer profile and addresses.

**Administration application**

1. Administration layout.
2. Dashboard.
3. Product management.
4. Category management.
5. Inventory management.
6. Order management.
7. Customer management.
8. Payment information.
9. Refund management.

Technical tasks:

- Configure React Router.
- Configure TanStack Query.
- Configure API client.
- Implement authentication state.
- Implement protected routes.
- Implement shared components.
- Implement form validation.
- Implement loading and error states.
- Add responsive layouts.

**Exit criteria:**

- Customer workflows work.
- Administration workflows work.
- API integration works.
- Authorization is respected.
- Responsive layouts work.
- Frontend tests pass.

Suggested commit:

```bash
git commit -m "feat(bio-com): implement React web application"
```

### Stage 17 — Integration and End-to-End Testing

**Goal:** Validate complete business workflows.

Important customer journey:

```text
Register
   |
   v
Login
   |
   v
Browse Products
   |
   v
Add to Cart
   |
   v
Checkout
   |
   v
Create Order
   |
   v
Process Payment
   |
   v
Fulfill Order
   |
   v
Ship Order
   |
   v
Confirm Delivery
```

Additional scenarios:

- Payment failure.
- Order cancellation.
- Inventory shortage.
- Partial shipment.
- Full refund.
- Partial refund.
- Unauthorized administration access.
- Duplicate payment requests.
- Duplicate order submissions.

**Exit criteria:**

- Main customer journey passes.
- Administration scenarios pass.
- Failure scenarios pass.
- Integration tests pass.
- End-to-end tests pass.

Suggested commit:

```bash
git commit -m "test(bio-com): cover end-to-end business workflows"
```

### Stage 18 — Security and Production Readiness

**Goal:** Prepare the platform for production deployment.

Tasks:

- Review authentication.
- Review authorization.
- Review session security.
- Review input validation.
- Review SQL injection protection.
- Review CSRF and XSS protections.
- Review rate limiting.
- Review abuse protection.
- Review secret management.
- Review audit logging.
- Review error handling.
- Review database backup and recovery.
- Review observability.
- Review performance.
- Review deployment configuration.
- Review operational documentation.

Production observability should include:

- Structured logs.
- Request correlation IDs.
- Health checks.
- Readiness checks.
- Error metrics.
- Database connection metrics.
- Background job metrics.

**Exit criteria:**

- Security tests pass.
- No known critical vulnerabilities remain.
- Backup and recovery procedures are tested.
- Operational health checks work.
- Performance meets agreed targets.
- Deployment procedures are documented.

Suggested commit:

```bash
git commit -m "chore(bio-com): prepare platform for production"
```

## 5. Implementation Rules

Every implementation step must follow this workflow:

1. Explain the problem being solved.
2. Explain the concepts and design decisions.
3. Define the expected behavior.
4. Design the domain model where applicable.
5. Define the application use case.
6. Implement infrastructure and persistence.
7. Expose the required API.
8. Write automated tests.
9. Run quality checks.
10. Update documentation.
11. Provide a Git commit message.

Do not move to the next step while the current step has failing tests or unresolved architectural decisions.

## 6. Testing Strategy

Testing must be introduced from the beginning, not postponed until the end.

| Test level | Responsibility |
|---|---|
| Unit tests | Domain rules and individual functions |
| Application tests | Use cases and orchestration |
| Integration tests | PostgreSQL and infrastructure |
| API tests | HTTP contracts and authorization |
| Architecture tests | Dependency boundaries |
| Frontend tests | Components and interactions |
| E2E tests | Complete business workflows |
| Security tests | Authentication and authorization |
| Performance tests | Database and API behavior |

Every business module must have its own meaningful tests.

## 7. Git Strategy

Use small, focused commits.

Suggested prefixes:

| Prefix | Purpose |
|---|---|
| feat | New functionality |
| fix | Bug fixes |
| test | Tests |
| docs | Documentation |
| refactor | Code restructuring |
| chore | Configuration and maintenance |

Every suggested commit message must include `bio-com`.

Examples:

```bash
git commit -m "feat(bio-com): add product creation use case"
git commit -m "test(bio-com): add inventory reservation tests"
git commit -m "docs(bio-com): document order lifecycle"
```

## 8. Definition of Done

A feature is complete when:

- Domain rules are implemented.
- Application use cases work.
- Infrastructure is implemented where required.
- API contracts are documented.
- Authorization is enforced.
- Unit tests pass.
- Integration tests pass where applicable.
- Error handling is implemented.
- Logging is available.
- Documentation is updated.
- Code formatting and linting pass.
- Git changes are committed.

## 9. Final Delivery

The final platform should provide:

**Customer capabilities**

- Registration and authentication.
- Product browsing.
- Shopping cart.
- Checkout.
- Orders.
- Payments.
- Shipping tracking.
- Refund information.
- Customer profile management.

**Administration capabilities**

- Product management.
- Inventory management.
- Order management.
- Customer management.
- Payment visibility.
- Refund management.

**Technical capabilities**

- Clean Architecture.
- DDD modular monolith.
- PostgreSQL persistence.
- Secure authentication and authorization.
- REST APIs.
- React web application.
- Rust data generator.
- Automated testing.
- Background processing.
- Observability.
- Production deployment documentation.

## 10. Delivery Philosophy

Build a working product incrementally.

Do not introduce distributed infrastructure until a real requirement justifies it.

Keep business rules in the domain and application layers.

Keep infrastructure replaceable.

Keep the frontend independent of backend implementation details.

Prioritize correctness, testability, security, and maintainability.

The objective is not simply to build an e-commerce website. It is to build a well-structured, production-ready software platform that demonstrates sound software architecture and engineering practices.
