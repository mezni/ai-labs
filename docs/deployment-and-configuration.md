# Deployment and Configuration

## 1. Purpose

This document defines how bio-com is configured and deployed.

It covers:

- Application configuration.
- Environment management.
- Environment variables.
- Secrets management.
- Docker.
- PostgreSQL configuration.
- Database migrations.
- Application startup.
- Graceful shutdown.
- Production deployment.
- Backup and recovery.

The goal is to make deployments predictable, repeatable, and easy to maintain.

## 2. Deployment Principles

The platform follows these principles:

### Keep Infrastructure Simple

Start with a small number of infrastructure components.

The initial deployment requires:

- bio-com application.
- PostgreSQL database.

No Kubernetes, Kafka, Redis, or service mesh is required initially.

### Environment Separation

Development, testing, staging, and production must have separate configurations and data.

### Configuration Outside the Code

Environment-specific values must not be hardcoded into the application.

### Reproducible Builds

The same application version should be deployable across environments.

### Secure Deployment

Secrets must never be committed to Git or included in public container images.

### Safe Database Changes

Database migrations must be applied in a controlled manner.

## 3. Deployment Architecture

Initial deployment:

```mermaid
flowchart TB
    Client[Web Client]
    Proxy[Reverse Proxy / HTTPS]
    
    subgraph Server["Application Host"]
        App["bio-com Rust Application"]
    end

    subgraph Database["Database Host or Container"]
        DB[(PostgreSQL)]
        Volume[(Persistent Database Volume)]
        DB --> Volume
    end

    Client --> Proxy
    Proxy --> App
    App --> DB
```

The application runs as one process containing the business modules.

PostgreSQL stores application data.

A reverse proxy handles external HTTPS traffic when required by the deployment environment.

## 4. Environments

The platform uses four environments.

| Environment | Purpose |
|---|---|
| Development | Local development |
| Test | Automated testing |
| Staging | Production-like validation |
| Production | Live customer traffic |

### Development

Developers run the Rust application locally.

PostgreSQL can run in Docker.

### Test

Automated tests use isolated test configuration and database instances.

Production customer data must never be used for testing.

### Staging

Staging should resemble production as closely as practical.

It is used to validate:

- Application configuration.
- Database migrations.
- API behavior.
- Security settings.
- Deployment procedures.

### Production

Production must use:

- Secure configuration.
- HTTPS.
- Persistent PostgreSQL storage.
- Automated backups.
- Restricted database access.
- Application health checks.
- Centralized log collection when available.

## 5. Configuration Management

Application configuration is provided through environment variables.

The application should load configuration once during startup and validate it before accepting requests.

Configuration should be represented using typed Rust structures.

Example:

```rust
pub struct AppConfig {
    pub environment: String,
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
}
```

Use a configuration loader such as `config` or a small custom implementation.

Validate required settings at startup.

If required configuration is missing or invalid, the application must fail to start.

Do not silently use insecure production defaults.

## 6. Environment Variables

Initial configuration:

| Variable | Description |
|---|---|
| APP_ENV | Application environment |
| APP_HOST | Application bind address |
| APP_PORT | Application port |
| DATABASE_URL | PostgreSQL connection URL |
| DATABASE_MAX_CONNECTIONS | Maximum pool connections |
| DATABASE_ACQUIRE_TIMEOUT | Connection acquisition timeout |
| RUST_LOG | Logging configuration |
| SESSION_TTL_SECONDS | Session lifetime |
| SESSION_COOKIE_SECURE | Secure cookie configuration |
| CORS_ALLOWED_ORIGINS | Allowed frontend origins |

Additional provider-specific configuration can be added when integrations are implemented.

### Example `.env.example`

```dotenv
APP_ENV=development
APP_HOST=127.0.0.1
APP_PORT=8080

DATABASE_URL=postgres://bio_com:change_me@localhost:5432/bio_com
DATABASE_MAX_CONNECTIONS=10
DATABASE_ACQUIRE_TIMEOUT=5

RUST_LOG=info,bio_com=debug

SESSION_TTL_SECONDS=86400
SESSION_COOKIE_SECURE=false

CORS_ALLOWED_ORIGINS=http://localhost:3000
```

This file contains example values only.

Never put real credentials in `.env.example`.

## 7. The `.env` File

For local development, developers can use a `.env` file.

Example:

```dotenv
APP_ENV=development
DATABASE_URL=postgres://bio_com:local_password@localhost:5432/bio_com
```

Important rules:

- Add `.env` to `.gitignore`.
- Never commit production credentials.
- Commit `.env.example`.
- Do not copy local credentials into staging or production.
- Do not include `.env` in Docker images.

In production, inject configuration through the deployment environment or a secrets manager.

## 8. Docker

Docker provides a consistent development and deployment environment.

The initial setup uses two containers:

1. bio-com application.
2. PostgreSQL database.

### Docker Compose

Example development configuration:

```yaml
services:
  postgres:
    image: postgres:17
    container_name: bio-com-db
    environment:
      POSTGRES_DB: bio_com
      POSTGRES_USER: bio_com
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U bio_com -d bio_com"]
      interval: 5s
      timeout: 5s
      retries: 5

  app:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: bio-com-app
    environment:
      APP_ENV: development
      APP_HOST: 0.0.0.0
      APP_PORT: 8080
      DATABASE_URL: postgres://bio_com:${POSTGRES_PASSWORD}@postgres:5432/bio_com
      RUST_LOG: info,bio_com=debug
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy

volumes:
  postgres_data:
```

Use a local `.env` file to supply `POSTGRES_PASSWORD`.

For production, do not expose PostgreSQL publicly.

Pin container image versions and use a tested upgrade process rather than relying on floating tags.

## 9. Dockerfile

Use a multi-stage build.

The build stage compiles the Rust application.

The runtime stage contains only what is required to execute it.

Example:

```dockerfile
FROM rust:1 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/bio-com /app/bio-com

EXPOSE 8080

CMD ["/app/bio-com"]
```

Adapt the Dockerfile to the actual workspace structure.

For production:

- Pin the Rust toolchain.
- Use a minimal runtime image.
- Run as a non-root user.
- Include required runtime libraries and CA certificates.
- Do not copy build tools into the runtime image.
- Use image scanning and dependency checks.

## 10. Database Configuration

PostgreSQL is the primary database.

Configure:

- Database name.
- Database user.
- Database password.
- Connection limits.
- Connection pool size.
- Connection timeouts.
- Backup strategy.

### Connection Pool

Use a bounded SQLx connection pool.

The pool must be configured according to:

- Available PostgreSQL connections.
- Application instance count.
- Expected concurrent requests.
- Database resource limits.

Do not configure every application instance to consume the entire database connection budget.

### Database Access

Use a dedicated application database user.

The application user should have only the permissions required to execute the application.

Use a separate migration role for schema changes where practical.

## 11. Database Migrations

Database schema changes must be managed through versioned migrations.

Example structure:

```text
migrations/
├── 0001_create_users.sql
├── 0002_create_roles.sql
├── 0003_create_products.sql
├── 0004_create_orders.sql
└── 0005_create_payments.sql
```

Use SQLx migrations.

### Migration Rules

- Every schema change must have a migration.
- Migrations must be versioned.
- Never manually modify production schemas.
- Test migrations before deployment.
- Back up important production data before risky changes.
- Review destructive changes carefully.

### Deployment Sequence

1. Build the application image.
2. Validate the migration set.
3. Back up the database when required.
4. Apply migrations using a controlled migration job.
5. Deploy the application.
6. Verify readiness.
7. Run smoke tests.

Avoid running migrations independently from every application instance during startup.

For incompatible schema changes, use an expand-and-contract migration strategy.

## 12. Application Startup

The application must validate its environment before accepting requests.

Recommended startup sequence:

1. Load configuration.
2. Validate configuration.
3. Initialize structured logging.
4. Create the database connection pool.
5. Verify database connectivity.
6. Initialize application modules.
7. Register API routes.
8. Start the HTTP server.
9. Begin accepting requests.

If a critical dependency is unavailable, startup should fail or readiness should remain false according to the deployment policy.

## 13. Graceful Shutdown

The application must shut down safely.

When receiving a termination signal:

1. Stop accepting new requests.
2. Allow in-flight requests to finish within a configured deadline.
3. Stop accepting new background work.
4. Complete or safely release active operations.
5. Close database connections.
6. Exit cleanly.

Do not terminate active financial operations without considering their final state.

## 14. Health Checks

Expose two health endpoints.

### Liveness

```text
GET /health/live
```

Checks whether the process is running.

### Readiness

```text
GET /health/ready
```

Checks whether critical dependencies are available and the application is ready to receive traffic.

These endpoints must not expose credentials or internal infrastructure details.

## 15. Logging and Monitoring

The application writes structured logs to stdout.

The deployment environment is responsible for collecting and retaining them.

Monitor:

- Application availability.
- API latency.
- HTTP error rate.
- Database availability.
- Database connection pool usage.
- CPU and memory usage.
- Payment provider failures.

Follow the standards defined in `observability.md`.

## 16. Production Deployment

The initial production deployment should remain simple.

Recommended components:

- One or more bio-com application instances.
- PostgreSQL.
- HTTPS reverse proxy or managed ingress.
- Persistent database storage.
- Backup storage.

The application must be stateless where practical.

Persistent business data belongs in PostgreSQL rather than local container storage.

### Deployment Process

1. Build a release artifact.
2. Run automated tests.
3. Scan dependencies and container images.
4. Create a versioned container image.
5. Apply approved database migrations.
6. Deploy the application.
7. Verify health endpoints.
8. Execute smoke tests.
9. Monitor logs and metrics.

Use immutable image tags based on a release version or commit identifier.

Avoid deploying untested changes directly to production.

## 17. Backup and Recovery

PostgreSQL data must be backed up regularly.

The backup strategy should include:

- Scheduled backups.
- Secure backup storage.
- Encryption.
- Retention policies.
- Periodic restoration tests.
- Documented recovery procedures.

Define:

**RPO (Recovery Point Objective):** How much data loss is acceptable?

**RTO (Recovery Time Objective):** How long can the platform remain unavailable?

These values must be agreed upon according to business requirements.

A backup is not considered reliable until restoration has been tested.

## 18. Configuration Security

Production configuration must follow security requirements.

Rules:

- Never store credentials in Git.
- Never expose secrets through API endpoints.
- Restrict access to deployment configuration.
- Rotate compromised credentials.
- Use separate credentials for each environment.
- Restrict database network access.
- Use secure cookie settings in production.
- Enforce HTTPS.
- Disable development-only features in production.

Do not enable debug logging indiscriminately in production.

## 19. Deployment Testing

Before production deployment, verify:

### Application

- Application starts successfully.
- Configuration validation works.
- Health endpoints respond correctly.
- Graceful shutdown works.

### Database

- Connection pool initializes.
- Migrations apply successfully.
- Existing data remains consistent.
- Backup restoration works.

### Security

- Production cookies use Secure and HttpOnly.
- CORS allows only approved origins.
- Database is not publicly exposed.
- Secrets are not present in the image.

### Business Operations

- Registration and login work.
- Product listing works.
- Checkout works.
- Payment integration works in the appropriate test environment.
- Refund workflows work.

## 20. Initial Technology Decisions

| Responsibility | Technology |
|---|---|
| Language | Rust |
| HTTP server | Axum |
| Async runtime | Tokio |
| Database | PostgreSQL |
| Database access | SQLx |
| Migrations | SQLx migrations |
| Containerization | Docker |
| Local orchestration | Docker Compose |
| Configuration | Environment variables |
| Local secrets | `.env` |
| Production secrets | Deployment secret injection |
| Logging | tracing |
| Monitoring | Lightweight metrics |
| Deployment | Container-based |

## 21. Key Design Decisions

1. Deploy bio-com as a modular monolith.
2. Use PostgreSQL as the primary database.
3. Use Docker for consistent environments.
4. Use Docker Compose for local development.
5. Keep environment-specific configuration outside the code.
6. Use `.env` locally and never commit it.
7. Manage database changes through versioned migrations.
8. Run migrations as a controlled deployment step.
9. Use multi-stage Docker builds.
10. Run containers as non-root users.
11. Implement health checks and graceful shutdown.
12. Back up PostgreSQL regularly and test restoration.
13. Avoid Kubernetes and heavy infrastructure initially.

---

**Main principle:** Build once, configure per environment, deploy predictably, and always have a tested recovery strategy.
