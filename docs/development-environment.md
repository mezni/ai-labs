# Development Environment

## 1. Purpose

This document explains how to configure and run the `bio-com` platform in a local development environment.

The development environment includes:

- Rust backend.
- React web application.
- PostgreSQL database.
- Database migrations.
- Rust data generator.
- Automated tests.

Developers should be able to start the platform, generate sample data, and test customer and administrator workflows without relying on external infrastructure.

## 2. Technology Stack

| Component | Technology | Purpose |
|---|---|---|
| Backend | Rust | Business logic and REST API |
| Frontend | TypeScript / React | Web application |
| Frontend tooling | Vite | Development server |
| Database | PostgreSQL | Persistent storage |
| Database access | SQLx | Rust database integration |
| Migrations | SQLx migrations | Database schema management |
| Data generator | Rust | Sample data generation |
| Package management | Cargo / npm | Dependency management |
| Testing | Cargo test / Vitest / Playwright | Automated testing |

## 3. Local Architecture

The initial development environment uses three main processes and one supporting tool.

```text
Developer Machine
│
├── PostgreSQL
│   └── localhost:5432
│
├── Rust Backend
│   └── localhost:8080
│
├── React Application
│   └── localhost:5173
│
└── Data Generator
    └── Connects to PostgreSQL
```

The Rust backend communicates with PostgreSQL.

The React application communicates with the Rust backend.

The data generator connects directly to PostgreSQL for development and test data creation.

## 4. Prerequisites

Install the following tools:

| Tool | Purpose |
|---|---|
| Rust | Backend and generator development |
| Cargo | Rust package management |
| rustfmt | Code formatting |
| Clippy | Rust linting |
| Node.js LTS | Frontend development |
| npm | Frontend dependencies |
| PostgreSQL | Database |
| Git | Version control |

Optional:

- Docker for running PostgreSQL locally.
- An API client for testing REST endpoints.
- A database management tool.

Use compatible tool versions across the development team.

## 5. Environment Variables

The backend and frontend have separate configuration files.

### Backend

Example `apps/api/.env.example`:

```env
APP_ENV=development
APP_HOST=127.0.0.1
APP_PORT=8080

DATABASE_URL=postgres://bio_com:local_password@localhost:5432/bio_com

LOG_LEVEL=debug
```

### Frontend

Example `apps/web/.env.example`:

```env
VITE_API_BASE_URL=http://localhost:8080/api/v1
```

### Data Generator

The data generator uses the development database connection.

It should read its configuration from an explicitly supplied environment file or environment variables.

Never commit actual credentials.

## 6. Database Setup

PostgreSQL is the primary database.

For local development, it can run directly on the machine or inside a single Docker container.

Example Docker command:

```bash
docker run -d \
  --name bio-com-postgres \
  -e POSTGRES_USER=bio_com \
  -e POSTGRES_PASSWORD=local_password \
  -e POSTGRES_DB=bio_com \
  -p 5432:5432 \
  -v bio-com-postgres-data:/var/lib/postgresql/data \
  postgres:17
```

For production deployments, use managed secrets and secure database configuration.

Do not reuse local credentials in production.

## 7. Database Migrations

Database migrations manage schema changes.

All schema changes must be version-controlled.

Example commands:

```bash
# Run migrations
sqlx migrate run --source migrations

# Check migration status
sqlx migrate info --source migrations
```

Migration files should follow a consistent naming convention.

Example:

```text
migrations/
├── 0001_create_identity_tables.sql
├── 0002_create_catalog_tables.sql
├── 0003_create_inventory_tables.sql
├── 0004_create_cart_tables.sql
├── 0005_create_order_tables.sql
└── 0006_create_payment_tables.sql
```

Migration ownership and ordering should follow the agreed domain and database design.

Never modify an already-applied migration in a shared environment. Create a new migration instead.

## 8. Running the Rust Backend

The backend runs independently from the frontend.

Example:

```bash
cargo run -p api
```

Expected address:

```text
http://localhost:8080
```

The API should expose health endpoints.

Example:

```text
GET /health
GET /ready
```

The health endpoint indicates whether the application is running.

The readiness endpoint indicates whether required dependencies, such as PostgreSQL, are available.

## 9. Running the React Application

Install frontend dependencies:

```bash
cd apps/web
npm install
```

Start the development server:

```bash
npm run dev
```

Expected address:

```text
http://localhost:5173
```

The frontend communicates with the Rust backend through the configured API base URL.

Configure CORS appropriately for local development, or use a Vite proxy.

## 10. Running the Data Generator

The data generator populates the development database.

Example:

```bash
cargo run -p data-generator -- generate \
  --profile small \
  --seed 42
```

This creates sample customers, products, orders, payments, and other related records.

Always run database migrations before generating data.

The generator must reject production database configurations.

## 11. Recommended Startup Order

Follow this order when starting the platform.

1. Start PostgreSQL.
2. Run database migrations.
3. Start the Rust API.
4. Generate development data.
5. Start the React application.
6. Open the web application in a browser.

The data generator can also be executed independently whenever additional sample data is required.

## 12. Testing

Different parts of the application have separate test commands.

### Backend

```bash
cargo test --workspace
```

### Frontend

```bash
cd apps/web
npm test
```

### Frontend production build

```bash
npm run build
```

### End-to-End Tests

```bash
cd apps/web
npx playwright test
```

Integration tests should use a dedicated test database, not the development database.

Tests must not depend on manually created local data.

## 13. Code Quality

Before committing changes, run the relevant quality checks.

### Rust

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### TypeScript

```bash
cd apps/web

npx tsc --noEmit
npm run lint
npm test
npm run build
```

The exact npm scripts should be defined in `package.json`.

## 14. Development Workflow

The normal development workflow is:

1. Pull the latest code.
2. Check database migration changes.
3. Start PostgreSQL.
4. Apply migrations.
5. Generate sample data if needed.
6. Start the backend.
7. Start the frontend.
8. Implement the required feature.
9. Add or update tests.
10. Run quality checks.
11. Commit the changes.

Backend and frontend changes should be coordinated through documented API contracts.

## 15. Database Reset

Developers sometimes need a clean database.

The reset process should:

1. Verify the target environment.
2. Require explicit confirmation.
3. Drop or recreate the development database.
4. Apply migrations.
5. Generate a fresh dataset.

Example:

```bash
cargo run -p data-generator -- reset --confirm
cargo run -p data-generator -- generate --profile small --seed 42
```

Reset operations must be disabled for production environments.

## 16. Infrastructure Decisions

The initial development environment intentionally avoids unnecessary services.

| Component | Initial decision |
|---|---|
| Database | PostgreSQL |
| Cache | Not required initially |
| Message broker | Not required initially |
| Distributed tracing backend | Not required initially |
| Container orchestration | Not required initially |
| API gateway | Not required initially |
| Object storage | Add when required |
| Background processing | Rust worker when required |

The objective is to establish a working modular monolith before introducing distributed infrastructure.

## 17. Development Environment Checklist

A developer should be able to verify the following:

- PostgreSQL is running.
- Database connection works.
- Migrations execute successfully.
- Rust API starts.
- API health checks pass.
- React application starts.
- Frontend can communicate with the API.
- Data generator creates valid sample data.
- Backend tests pass.
- Frontend tests pass.
- Production frontend build succeeds.

## 18. Future Improvements

Potential improvements include:

- A Docker Compose configuration for PostgreSQL.
- A root-level development script.
- Automated database initialization.
- Seeded test fixtures.
- CI pipelines.
- Local API documentation.
- Development observability.
- Separate staging environment.

Introduce these improvements only when they provide clear value.
