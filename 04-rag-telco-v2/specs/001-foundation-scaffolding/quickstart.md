# Quickstart: Foundation & Scaffolding

**Date**: 2026-09-08 | **Feature**: 001-foundation-scaffolding

This is the runnable validation guide proving the Phase 0 foundation works end-to-end
(SC-001 through SC-005). It uses links to the contracts and data model rather than
duplicating detail; implementation specifics live in `tasks.md`.

## Prerequisites

- Container runtime (per ARCHITECTURE.md / RUNBOOK.md) — required dependency.
- An operational database target (fresh or disposable) for migration validation
  (data-model.md).
- Secret values prepared locally — never committed:
  `DATABASE_URL`, `OPENROUTER_API_KEY`, `JWT_SECRET`, `LANGFUSE_PUBLIC_KEY`,
  `LANGFUSE_SECRET_KEY` (see `.env.example`).

## Setup

1. Prepare the environment from the non-committed template (do not commit real values).
2. Validate connectivity to the database target.
3. Start all baseline services (frontend, api, postgres, prometheus, grafana, langfuse):

   ```bash
   docker compose up -d
   ```

4. Confirm all services are running:

   ```bash
   docker compose ps
   ```

   Expected: `frontend`, `api`, `postgres`, `prometheus`, `grafana`, `langfuse` all
   `running`.

## Validation Scenarios

### Scenario 1 — Health endpoints (SC-001, SC-002)

Verify the service is alive:

```bash
curl http://localhost:8000/api/v1/health
```

Expected:

```json
{ "status": "UP" }
```

Verify readiness reflects local components and reports the external AI dependency
separately (contract: `contracts/api-contracts.md`). With the AI provider down, readiness
still returns READY for local platform while `llm` shows separate status:

```bash
curl http://localhost:8000/api/v1/health/ready
```

Expected (example when external AI unreachable):

```json
{
  "status": "READY",
  "database": "UP",
  "llm": "DEGRADED"
}
```

### Scenario 2 — Schema migrations (SC-003, FR-008)

Apply migrations to a fresh database and confirm the full data model is created
(data-model.md) with no error:

```bash
alembic upgrade head
```

Expected: all documented tables and indexes created; migration history tracked; applying
again is a no-op (idempotent). A subsequent migration applies only the new change.

### Scenario 3 — Baseline CI on every branch/PR (SC-004, FR-007)

Trigger the CI pipeline (build, test, security) by pushing to any branch or opening a
pull request:

Expected: pipeline stages (build, test, security) complete without failure. A failing
change marks the run failed and surfaces it to the developer.

## Expected Outcomes

- All services start and remain running via a single documented command (SC-001).
- Readiness returns READY/UP for local components with external AI reported separately
  (SC-002).
- 100% of the documented data model tables and indexes are created on a fresh database
  without error (SC-003).
- CI runs on every branch/PR and passes build, test, and security stages (SC-004).
- An operator reaches a fully healthy baseline from the documented setup steps in under
  30 minutes (SC-005).
