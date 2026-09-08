# Tasks: Foundation & Scaffolding

**Input**: Design documents from `/specs/001-foundation-scaffolding/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Web app**: `backend/app/`, `frontend/src/` (per plan.md Project Structure)
- Docker/compose and infra config live at repository root (`docker-compose.yml`, `infra/`)
- CI workflows live under `.github/workflows/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create the repository directory structure (frontend/, backend/, scripts/, prompts/, evaluations/, infra/, .github/workflows/) per plan.md Project Structure
- [X] T002 Initialize backend Python project with FastAPI dependency definitions in backend/pyproject.toml
- [X] T003 [P] Initialize frontend React/Next.js project in frontend/ with dependency manifest (package.json)
- [X] T004 [P] Configure linting and formatting tooling for backend (ruff) in backend/pyproject.toml
- [X] T005 [P] Configure linting and formatting tooling for frontend (eslint/prettier) in frontend/package.json
- [X] T006 Create the container orchestration entrypoint docker-compose.yml at repository root defining all baseline services (frontend, api, postgres, prometheus, grafana, langfuse)
- [X] T007 Create the non-committed environment template .env.example at repository root with placeholders for DATABASE_URL, OPENROUTER_API_KEY, JWT_SECRET, LANGFUSE_PUBLIC_KEY, LANGFUSE_SECRET_KEY

**Checkpoint**: Project structure and docker compose skeleton in place.

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 Create the backend FastAPI application entry point in backend/app/main.py that mounts the /api/v1 router structure
- [X] T009 [P] Configure environment configuration management in backend/app/core/config.py that reads .env variables
- [X] T010 [P] Set up the PostgreSQL + pgvector service configuration in infra/postgres/ (init script enabling the vector extension)
- [X] T011 [P] Create baseline observability service configs in infra/prometheus/ and infra/grafana/ (minimal scrape/dashboard placeholders)
- [X] T012 Create the backend API router registration structure in backend/app/api/ with an operational router for health endpoints

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

## Phase 3: User Story 1 - Provision a Runnable Platform Baseline (Priority: P1) 🎯 MVP

**Goal**: All baseline services start together via a single documented command and report a healthy readiness status (SC-001, SC-002).

**Independent Test**: Run `docker compose up -d`, then `docker compose ps` and query the readiness endpoint to confirm all local components report READY/UP. (See quickstart.md Scenario 1.)

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement the liveness endpoint in backend/app/api/routes/health.py (GET /api/v1/health returns { "status": "UP" })
- [X] T014 [P] [US1] Implement the liveness detail endpoint in backend/app/api/routes/health.py (GET /api/v1/health/live returns { "status": "ALIVE" })
- [X] T015 [US1] Implement the readiness endpoint in backend/app/api/routes/health.py (GET /api/v1/health/ready) that reflects local components (application + database) and reports the external AI provider as a separate `llm` status without failing the overall READY result (contracts/api-contracts.md)
- [X] T016 [US1] Wire the operational health router into the application startup in backend/app/main.py
- [X] T017 [US1] Configure the framework services to start as one unit in docker-compose.yml with correct service dependencies
- [X] T018 [US1] Add logging of health-check events in backend/app/api/routes/health.py to support observability (constitution Principle IV)
- [X] T034 [US1] Create a minimal runnable frontend skeleton in frontend/src/app (root App/Home page + Dockerfile) so the UI service builds and serves a health/landing indicator

**Checkpoint**: User Story 1 is functional - local components (API + UI) start together and readiness returns READY/UP with external AI reported separately.

## Phase 4: User Story 2 - Initialize Database Schema & Migrations (Priority: P1)

**Goal**: The full documented data model is created through version-controlled migrations that apply cleanly on a fresh environment (SC-003, FR-004, FR-008).

**Independent Test**: Apply migrations to a fresh database via `alembic upgrade head` and confirm all tables/indexes created with no error, and history tracked. (See quickstart.md Scenario 2.)

### Implementation for User Story 2

- [X] T019 [P] [US2] Initialize the Alembic migration environment in backend/alembic/ (alembic.ini, env.py wired to the application config)
- [X] T020 [P] [US2] Create the ORM models package in backend/app/models/ defining User, Session, Document, DocumentVersion, Chunk, Conversation, Question, Answer, Citation, Feedback, LLMUsage, RetrievalLog, AuditLog, ApplicationLog, Trace, MetricsSnapshot, and cost-summary models per data-model.md
- [X] T021 [US2] Write the initial Alembic migration in backend/alembic/versions/ that creates the full data model (all tables, indexes, and the pgvector extension) per data-model.md
- [X] T022 [US2] Add the vector embedding column and the IVFFlat vector index on the chunks table in the initial migration per data-model.md (Indexing Strategy)
- [X] T023 [US2] Add the GIN full-text index on chunks(content) in the migration per data-model.md (Indexing Strategy)
- [X] T024 [US2] Add relational indexes (documents.category, document_versions.status, questions.predicted_category) and the unique constraint on users.email in the migration per data-model.md
- [X] T025 [US2] Implement the database connection/engine setup in backend/app/core/db.py used by Alembic and the application

**Checkpoint**: User Story 2 is functional - schemas create cleanly on a fresh database with tracking; ready for ingestion/retrieval in later phases.

## Phase 5: User Story 3 - Establish a Baseline CI Pipeline (Priority: P2)

**Goal**: A baseline CI pipeline runs on every branch and pull request performing build, test, and security validation (SC-004, FR-007).

**Independent Test**: Push to any branch or open a PR and confirm the pipeline's build, test, and security stages run and complete without failure. (See quickstart.md Scenario 3.)

### Implementation for User Story 3

- [X] T026 [P] [US3] Create the build workflow in .github/workflows/build.yml that builds the backend and frontend on every branch and pull request
- [X] T027 [P] [US3] Create the test workflow in .github/workflows/test.yml that runs backend (pytest) and frontend test checks on every branch and pull request
- [X] T028 [P] [US3] Create the security workflow in .github/workflows/security.yml that runs dependency, SAST, container/image, and secret-detection scans on every branch and pull request (SECURITY.md Secure Delivery Gates)

**Checkpoint**: All user stories are independently functional.

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T029 [P] Write the quickstart validation guide content in specs/001-foundation-scaffolding/quickstart.md (start-up, health, migration, CI validation scenarios)
- [X] T030 [P] Add baseline structured JSON logging configuration in backend/app/core/observability.py used across health and startup paths
- [X] T031 Add README-level setup/start instructions at repository root (README.md) referencing .env.example and required runtime secrets
- [X] T032 Security hardening: assert no secrets committed in the repository (validate .env.example is the only credential template) per constitution Security requirements
- [X] T033 Run the quickstart.md validation scenarios end-to-end and confirm SC-001 through SC-005 are met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories (shares the postgres/pgvector config from Phase 2)
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Depends on backend/frontend projects existing (Setup) but is otherwise independent of US1/US2 implementations

### Within Each User Story

- Core infrastructure before implementation
- Health endpoints (US1) and migrations (US2) are independently testable
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (within Phase 1)
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, Users Story 1, 2, and 3 can start in parallel (if team capacity allows)
- Models within a story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all health endpoints for User Story 1 together:
Task: "Implement liveness endpoint in backend/app/api/routes/health.py"
Task: "Implement liveness detail endpoint in backend/app/api/routes/health.py"
Task: "Implement readiness endpoint in backend/app/api/routes/health.py"

# Then wire-up and docker-compose integration sequentially
Task: "Wire the operational health router into the application startup in backend/app/main.py"
Task: "Configure services to start as one unit in docker-compose.yml"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (runnable baseline + readiness)
4. **STOP and VALIDATE**: Test User Story 1 independently (quickstart Scenario 1)
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (runnable baseline) → Test independently → Deploy/Demo (MVP: bootable, healthy stack)
3. Add User Story 2 (schema + migrations) → Test independently → Deploy/Demo
4. Add User Story 3 (baseline CI) → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (provisioning + health)
   - Developer B: User Story 2 (migrations + data model)
   - Developer C: User Story 3 (CI workflows)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
