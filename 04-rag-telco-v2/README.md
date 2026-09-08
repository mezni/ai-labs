# Telecom Policy Assistant — Phase 0 Foundation

Scaffolded baseline for the Telecom Policy Assistant (docs in `docs/`): a containerized
web application (FastAPI backend + Next.js frontend) on PostgreSQL 16 + pgvector with a
baseline CI pipeline and health/readiness endpoints.

## Stack

- **Backend**: Python 3.11+ / FastAPI / SQLAlchemy / Alembic
- **Frontend**: React / Next.js (Node 20+)
- **Database**: PostgreSQL 16 with pgvector (single shared database)
- **Observability**: Prometheus, Grafana, Langfuse (baseline provisioning)
- **CI**: GitHub Actions (build, test, security)

## Prerequisites

- Docker with Compose
- Python 3.11+ (local backend runs) / Node 20+ (local frontend runs)

## Setup

1. Prepare the environment from the template (never commit real values):

   ```bash
   cp .env.example .env
   # fill in DATABASE_URL, OPENROUTER_API_KEY, JWT_SECRET, LANGFUSE_* values
   ```

2. Start all baseline services with a single command (FR-001 / SC-001):

   ```bash
   docker compose up -d
   ```

3. Confirm all services are running:

   ```bash
   docker compose ps
   ```

## Health (readiness)

Readiness reflects local platform components; the external AI provider is reported
separately (FR-002 / SC-002):

```bash
curl http://localhost:8000/api/v1/health        # -> {"status":"UP"}
curl http://localhost:8000/api/v1/health/live   # -> {"status":"ALIVE"}
curl http://localhost:8000/api/v1/health/ready  # -> {"status":"READY","database":"UP","llm":"DEGRADED"}
```

## Database migrations

```bash
cd backend
alembic upgrade head        # creates the full data model on a fresh database
alembic current             # shows tracked migration history
```

## CI

Push to any branch or open a pull request to trigger the baseline pipeline
(build, test, security scans).

## Validation

See `specs/001-foundation-scaffolding/quickstart.md` for the end-to-end validation
scenarios (SC-001 through SC-005).