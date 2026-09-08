# API Contracts: Foundation & Scaffolding

**Date**: 2026-09-08 | **Feature**: 001-foundation-scaffolding

Phase 0 exposes the operational (health) endpoints of the FastAPI service under the
`/api/v1` base URL per RUNBOOK.md (Section 10). These are the primary externally-visible
interfaces realized in this foundation phase. Business endpoints (chat, search,
documents, reports) are scoped to later phases and documented in `docs/API_SPEC.md`.

## Base URL

All endpoints are relative to the base URL, e.g. `GET /health` resolves to
`http://localhost:8000/api/v1/health` (RUNBOOK.md Section 10).

## GET /health

Liveness check for the application process.

**Response 200**:

```json
{ "status": "UP" }
```

## GET /health/live

Liveness check indicating the service is running.

**Response 200**:

```json
{ "status": "ALIVE" }
```

## GET /health/ready

Readiness check. Per the clarified spec (FR-002), readiness reflects **local
platform components** (application + database). The external AI provider is reported as a
**separate** status and does NOT fail the overall check when unreachable.

**Response 200**:

```json
{
  "status": "READY",
  "database": "UP",
  "llm": "DEGRADED"
}
```

### Fields

| Field      | Meaning                                                        |
|------------|----------------------------------------------------------------|
| status     | `READY` when all local components are available; reflects local readiness only |
| database   | `UP` / `DOWN` for the local database dependency                 |
| llm        | `UP` / `DEGRADED` / `DOWN` — status of the external AI provider, reported separately |

### Semantics

- `status` = `READY` when the application and local database are available.
- `llm` reports the external AI provider state (e.g., `DEGRADED` when unreachable) but
  does not change the overall `status` from `READY`.

## READY / UP Status

Both the `/health` and `/health/ready` endpoints are the verification target of
User Story 1 and SC-002: an operator queries readiness and receives a READY / UP status
indicating the platform baseline is available while still surfacing external AI health
separately.

**Out of scope for Phase 0 contracts**: authentication-gated business endpoints, request
ID/correlation headers propagation, and full response schema envelope
(`{"success": true, "data": {}}`). These are introduced with the API layer in Phase 3 per
API_SPEC.md and are noted here only as forward references.
