# Research: Foundation & Scaffolding

**Date**: 2026-09-08 | **Feature**: 001-foundation-scaffolding

## Overview

Phase 0 establishes the runnable baseline for the Telecom Policy Assistant. All
technical decisions below are resolved from the authoritative project documents
(ARCHITECTURE.md, EXECUTION_PLAN.md, DATA_MODEL.md, RUNBOOK.md, SECURITY.md), so there
were no unresolved NEEDS CLARIFICATION items in the Technical Context.

## Decisions

### Decision 1: Containerized web application with docker compose

- **Decision**: Provision the stack as a set of containers (frontend, api, database,
  metrics, tracing, dashboards) orchestrated by a single docker compose configuration,
  started/reliably stopped via `docker compose`.
- **Rationale**: ARCHITECTURE.md (Section 13) and RUNBOOK.md (Sections 7-9) define
  container layout and start-up/shutdown via docker compose. It satisfies FR-001 (all
  services start together via a single documented command) and simplifies local
  development parity with deployment.
- **Alternatives considered**: Manual per-service startup (more error-prone, no parity
  with deployment); Kubernetes (too heavy for a development baseline).

### Decision 2: Single shared PostgreSQL database with pgvector

- **Decision**: Keep the full data model and document embeddings in one PostgreSQL
  database using the vector extension; version the schema with Alembic migrations.
- **Rationale**: DATA_MODEL.md (Section 12-13) specifies all tables in PostgreSQL with
  pgvector for embeddings and FTS/vector indexes. A single shared database matches the
  documented architecture, keeps migrations and CI simple, and enables the "one ACTIVE
  version" and retrieval constraints. Resolves clarification (all data + embeddings in
  one DB).
- **Alternatives considered**: Separate vector store (adds operational complexity and
  diverges from documented architecture); deferring core entities (causes schema churn
  later, explicitly rejected in the spec Assumptions).

### Decision 3: Readiness reflects local components; external AI reported separately

- **Decision**: The readiness endpoint returns READY / UP based on locally-managed
  platform components (application + database), and reports the external AI dependency
  (OpenRouter/LLM) as a separate status without failing the overall check when down.
- **Rationale**: RUNBOOK.md (Section 10) defines the readiness contract; treating the
  platform as unavailable when only the external provider is down would produce
  misleading outages. Resolves clarification.
- **Alternatives considered**: Fail readiness when external AI is down (flags a
  platform-available-but-provider-down state as an outage); no dependency awareness
  (loses visibility).

### Decision 4: Baseline CI on every branch and pull request

- **Decision**: The CI pipeline (build, test, security) runs on every branch and pull
  request, providing early feedback before changes reach the main branch.
- **Rationale**: RUNBOOK.md Appendix A stages lint → unit tests → security scan → build
  → integration → evaluation → quality gate. Running on all branches catches failures
  early and matches the governed delivery pipeline. Resolves clarification.
- **Alternatives considered**: Main-branch-only (delays failure detection); PR-only
  (misses direct-branch commits).

### Decision 5: Alembic as the version-controlled migration mechanism

- **Decision**: Use Alembic to create and evolve the PostgreSQL schema, with migration
  history tracked in the repository.
- **Rationale**: EXECUTION_PLAN.md Phase 0 explicitly requires "Alembic migrations for
  all DATA_MODEL tables" and "Schema migrations apply cleanly." Satisfies FR-004 and
  FR-008. Data model follows DATA_MODEL.md Section 12 schema.
- **Alternatives considered**: Raw SQL scripts (no incremental tracking); ORM auto-create
  (unsafe for production evolution, violates FR-008).

### Decision 6: Secrets via non-committed environment template

- **Decision**: Required secrets (database connection, AI service key, session secret,
  monitoring keys) are declared in a documented `.env.example` template that is committed,
  while actual values are provided at runtime and never stored in source control.
- **Rationale**: SECURITY.md (Sections 9, 12) prohibits secrets in source/git and
  mandates env-var storage in development. EXECUTION_PLAN Phase 0 lists the required
  variables. Satisfies FR-005 and the constitution's Security requirements.
- **Alternatives considered**: Committing real secrets (prohibited); secret manager-only
  (overkill for the development baseline; used in later phases).

## Requirements Traceability

| Requirement | Addressed By |
|-------------|--------------|
| FR-001 (all services together) | Decision 1 (docker compose) |
| FR-002 (readiness, local + external separate) | Decision 3 |
| FR-003 (single shared DB + embeddings) | Decision 2 |
| FR-004 (versioned migrations, full data model) | Decision 5 |
| FR-005 (non-committed env template) | Decision 6 |
| FR-006 (repo scaffold) | Project Structure in plan.md |
| FR-007 (CI on every branch/PR) | Decision 4 |
| FR-008 (clean fresh migrations, history tracked) | Decision 5 |

## Gate Status

All constitution gates pass (see Constitution Check in plan.md). No violations require
Complexity Tracking.
