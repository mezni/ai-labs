# Architecture Decisions

## 1. Purpose

This document records the main technical decisions for bio-com.

It explains:

- What we decided.
- Why we made the decision.
- What alternatives we considered.
- What trade-offs we accept.

These decisions are called Architecture Decision Records (ADRs).

An ADR helps the team understand why the system is designed in a particular way. It also prevents us from repeatedly debating decisions that have already been made.

## 2. Architecture Decision Principles

When making architecture decisions, follow these principles:

1. Solve the current business problem first.
2. Prefer simple solutions over unnecessary infrastructure.
3. Keep business rules independent of technical frameworks.
4. Make modules independent and clearly defined.
5. Protect data consistency.
6. Introduce complexity only when there is a real need.
7. Document important trade-offs.
8. Allow decisions to evolve when requirements change.

## 3. Current Architecture Decisions

The following table summarizes the main decisions.

| Area | Decision |
|---|---|
| Project name | bio-com |
| Programming language | Rust |
| Architecture style | Modular monolith |
| Domain modeling | Domain-Driven Design (DDD) |
| Code organization | Clean Architecture |
| API framework | Axum |
| Database | PostgreSQL |
| Database access | SQLx |
| API style | REST |
| Authentication | Dedicated Identity & Access module |
| Authorization | Role-Based Access Control (RBAC) |
| Password hashing | Argon2id |
| Session storage | PostgreSQL |
| Frontend | To be defined |
| Deployment | Docker |
| Initial infrastructure | Lightweight |
| Messaging | No message broker initially |
| Caching | No distributed cache initially |

## 4. ADR-001: Use Rust

<badge color="success">Accepted</badge>

### Context

bio-com needs a reliable backend that can support business operations such as:

- Product management.
- Customer accounts.
- Orders.
- Payments.
- Inventory.
- Refunds.

The backend should provide good performance, strong type safety, and predictable resource usage.

### Decision

Use Rust as the primary backend programming language.

### Alternatives Considered

- Python.
- Java.
- Go.
- TypeScript.

### Why Rust?

Rust provides:

- Memory safety without a garbage collector.
- Strong compile-time checks.
- Good performance.
- Explicit error handling.
- Strong type modeling.
- Reliable concurrency support.

### Trade-offs

Rust has a steeper learning curve than some alternatives.

Development may require more explicit types and error handling, but these practices can improve reliability.

## 5. ADR-002: Use a Modular Monolith

<badge color="success">Accepted</badge>

### Context

The platform contains several business capabilities:

- Identity and Access.
- Catalog.
- Product.
- Cart.
- Order.
- Payment.
- Inventory.
- Shipping.

Starting with microservices would introduce additional infrastructure and operational complexity.

### Decision

Build bio-com as a modular monolith.

Each business module has its own responsibilities and clear boundaries.

All modules run in one application and are deployed together.

### Why?

A modular monolith provides:

- Simpler deployment.
- Easier local development.
- Straightforward transactions.
- Lower infrastructure costs.
- Clear module boundaries.
- A practical path for future extraction if needed.

### Trade-offs

Modules share the same deployment lifecycle.

A problem in one module can affect the whole application if not properly isolated.

### Important Rule

Modules must not access another module's internal implementation directly.

They should communicate through defined application interfaces.

## 6. ADR-003: Use Domain-Driven Design

<badge color="success">Accepted</badge>

### Context

E-commerce contains different business concepts and rules.

For example:

- A Product describes what can be sold.
- Inventory tracks available stock.
- An Order represents a customer's purchase.
- A Payment records the financial operation.

These concepts should not be mixed into one large business model.

### Decision

Use Domain-Driven Design to organize the business logic.

### Main Concepts

- Domain: the business problem we are solving.
- Subdomain: a specific part of the business.
- Bounded Context: a boundary where a domain model has a clear meaning.
- Entity: an object with a unique identity.
- Value Object: an object defined by its values.
- Aggregate: a consistency boundary for domain changes.
- Domain Event: a record that something important happened.
- Repository: an abstraction for loading and saving domain objects.

### Trade-offs

DDD requires careful modeling and clear boundaries.

It can introduce unnecessary complexity if applied to simple features without meaningful business rules.

Use DDD where it provides real value.

## 7. ADR-004: Use Clean Architecture

<badge color="success">Accepted</badge>

### Context

Business rules should not depend directly on web frameworks, databases, or external services.

For example, an order should not need to know whether its data is stored using PostgreSQL or another database.

### Decision

Use Clean Architecture with clear separation between:

- Domain.
- Application.
- Infrastructure.
- API.

### Dependency Rule

Dependencies should point inward.

```text
API
 |
 v
Application
 |
 v
Domain

Infrastructure implements interfaces
defined by the inner layers.
```

### Responsibilities

| Layer | Responsibility |
|---|---|
| Domain | Business rules and domain models |
| Application | Use cases and workflow orchestration |
| Infrastructure | Database and external integrations |
| API | HTTP endpoints and request handling |

### Trade-offs

The architecture introduces interfaces and additional types.

For small features, this may require more code than a simple direct implementation.

The benefit is that business logic remains easier to test and maintain.

## 8. ADR-005: Use PostgreSQL

<badge color="success">Accepted</badge>

### Context

The platform manages relational business data.

Examples include:

- Customers and their orders.
- Orders and their items.
- Payments and refunds.
- Products and inventory.

These relationships require reliable transactions and data integrity.

### Decision

Use PostgreSQL as the primary database.

### Why?

PostgreSQL provides:

- ACID transactions.
- Foreign keys.
- Unique constraints.
- Indexes.
- JSONB when needed.
- Reliable concurrency control.
- Strong SQL support.

### Trade-offs

The database becomes a shared infrastructure dependency for all modules.

Module ownership must be enforced through schema design, application boundaries, and database access conventions.

## 9. ADR-006: Use SQLx

<badge color="success">Accepted</badge>

### Context

The application needs database access while preserving control over SQL queries and database behavior.

### Decision

Use SQLx for PostgreSQL access.

### Why?

SQLx provides:

- Async database access.
- Compile-time checked queries when configured.
- Explicit SQL.
- Connection pooling.
- Transaction support.
- Migration support.

### Trade-offs

Developers need to understand SQL and database design.

This is intentional: important queries should remain visible and understandable.

## 10. ADR-007: Keep Modules in One Application

<badge color="success">Accepted</badge>

### Context

The system contains multiple business modules, but they do not currently require independent deployment.

### Decision

Keep the modules inside one Rust application.

Use module-level interfaces for communication.

### Example

An order workflow may need to check product availability.

It should call the inventory module's application interface rather than querying inventory tables directly.

### Trade-offs

The modules cannot be scaled independently at deployment level.

This is acceptable until real performance or organizational requirements justify a different approach.

## 11. ADR-008: Build Identity and Access as a Dedicated Module

<badge color="success">Accepted</badge>

### Context

The application needs authentication and authorization.

These responsibilities should be centralized rather than implemented separately in every business module.

### Decision

Create a dedicated Identity & Access module inside the modular monolith.

### Responsibilities

- User registration.
- Login and logout.
- Password hashing.
- Session management.
- Role management.
- Permission checks.
- Resource ownership checks.

### Initial Security Choices

- Argon2id for password hashing.
- PostgreSQL-backed sessions.
- Secure random session tokens.
- Store only session token hashes.
- Role-Based Access Control.
- Ownership checks for customer resources.

### Trade-offs

The application depends on this module for identity-related operations.

Its interfaces must remain stable and its security requirements must be tested carefully.

## 12. ADR-009: Avoid Heavy Infrastructure Initially

<badge color="success">Accepted</badge>

### Context

The first version should remain easy to develop, deploy, and operate.

Adding distributed infrastructure before it is needed would increase cost and maintenance.

### Decision

Do not introduce the following components initially:

- Kafka.
- RabbitMQ.
- Redis.
- Kubernetes.
- Service mesh.
- External identity platform.

### Initial Approach

| Requirement | Initial Solution |
|---|---|
| Persistent data | PostgreSQL |
| Sessions | PostgreSQL |
| Background work | In-process or scheduled jobs |
| Database migrations | SQLx |
| Application deployment | Docker |
| Configuration | Environment variables |
| Logging | Structured application logs |
| API security | Application middleware and policies |

### Trade-offs

The application has fewer distributed capabilities.

For example, in-process background work does not provide the durability or independent scaling of a dedicated job system.

Introduce additional infrastructure when measurable requirements justify it.

## 13. ADR-010: Use REST APIs

<badge color="success">Accepted</badge>

### Context

The platform needs a consistent way to expose business operations to frontend applications and external clients.

### Decision

Use REST APIs with JSON.

API versioning starts with:

```text
/api/v1
```

### Why?

REST is widely understood and integrates easily with web and mobile applications.

It also provides a straightforward foundation for OpenAPI documentation.

### Trade-offs

Some complex workflows may require multiple API requests.

Use clear resource boundaries and application use cases rather than introducing a different API style prematurely.

## 14. ADR-011: Use Axum for HTTP

<badge color="success">Accepted</badge>

### Context

The backend needs an asynchronous HTTP framework that integrates naturally with Rust.

### Decision

Use Axum.

### Why?

Axum provides:

- Async request handling.
- Typed extractors.
- Middleware integration through Tower.
- Clear routing.
- Integration with the Tokio ecosystem.

### Trade-offs

Developers must understand async Rust and the Tower ecosystem.

HTTP handlers should remain thin and delegate business operations to application services.

## 15. ADR-012: Use Docker for Deployment

<badge color="success">Accepted</badge>

### Context

The application needs a repeatable way to run in development and production environments.

### Decision

Use Docker to package the application.

Use Docker Compose for local development when running the application with PostgreSQL.

### Initial Setup

```text
bio-com
├── bio-com application container
└── PostgreSQL container
```

### Trade-offs

Docker adds image-building and deployment configuration.

It provides a consistent runtime without requiring Kubernetes.

## 16. When Should We Reconsider a Decision?

Architecture decisions are not permanent.

Revisit them when:

- Business requirements change.
- Performance measurements reveal a bottleneck.
- Deployment becomes difficult.
- A module needs independent scaling.
- The team grows significantly.
- Security requirements change.
- Operational costs become too high.
- Existing architecture prevents important business capabilities.

Do not introduce new infrastructure only because it is popular or widely used.

There should be a clear problem and a measurable reason.

## 17. ADR Template

Use this template for future architecture decisions.

```markdown
# ADR-XXX: Decision Title

## Status
Proposed | Accepted | Rejected | Superseded

## Context
What problem are we trying to solve?

## Decision
What did we decide?

## Alternatives Considered
What other options did we evaluate?

## Reasons
Why did we choose this option?

## Trade-offs
What benefits and disadvantages does this decision introduce?

## Consequences
What changes are required in the system?
```

## 18. ADR Storage

Keep individual decisions under:

```text
docs/
└── adr/
    ├── 001-use-rust.md
    ├── 002-modular-monolith.md
    ├── 003-domain-driven-design.md
    ├── 004-clean-architecture.md
    └── ...
```

This document provides the architecture overview.

Individual ADR files can be introduced when a decision needs more detailed discussion, history, or superseding decisions.

---

**Main principle:** Every important architecture decision should solve a real problem, have a clear reason, and make its trade-offs visible.
