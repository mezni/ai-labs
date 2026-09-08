# Data Model: Foundation & Scaffolding

**Date**: 2026-09-08 | **Feature**: 001-foundation-scaffolding

The authoritative logical and physical schema is documented in the repository
`docs/DATA_MODEL.md`. This file captures the Phase 0 data model as it must be realized
by the Alembic migrations (FR-004, FR-008), with validation rules derived from the
feature spec and constitution.

## Storage Decision

Single shared PostgreSQL database with vector capability (pgvector) for document
embeddings. Applies the full documented data model now to avoid incompatible schema
churn when ingestion/retrieval arrive in later phases.

## Entities

### Identity & Access

- **users** — authenticated platform user (id, email, display_name, role, store_id,
  status). Unique email. Roles: RETAIL_AGENT, SUPERVISOR, ADMIN, SYSTEM.
- **sessions** — authenticated session (id, user_id, started_at, ended_at, ip_address,
  user_agent).

### Knowledge Base

- **documents** — business policy document (id, name, category, owner, status). Category
  in {BILLING, ROAMING, USAGE, LEGAL, CONTRACT, PROMOTION, PROCEDURE, DEVICE,
  SIM_MANAGEMENT, OTHER}.
- **document_versions** — version of a document (id, document_id, version, status,
  effective_date, expiration_date, uploaded_at). Status in {ACTIVE, DRAFT, RETIRED,
  ARCHIVED}; exactly one ACTIVE per document family.
- **chunks** — retrieval unit (id, document_version_id, page_number, section,
  subsection, content, token_count, embedding). Embedding is a vector dimension 1024.
- **embedding** — the vector representation stored on the chunk via pgvector.

### Chat Domain

- **conversations** — chat thread (id, user_id, title).
- **questions** — user request (id, conversation_id, question_text,
  predicted_category).
- **answers** — generated answer (id, question_id, answer_text, confidence_score,
  model_name).
- **citations** — link answer to supporting chunks (id, answer_id, chunk_id,
  document_name, page_number, section).
- **feedback** — user evaluation (id, answer_id, user_id, rating, comment). Rating in
  {HELPFUL, NOT_HELPFUL}.

### FinOps & Observability

- **llm_usage** — model invocation (id, question_id, user_id, store_id, model_name,
  input_tokens, output_tokens, total_tokens, latency_ms, estimated_cost).
- **daily_cost_summary / store_cost_summary** — aggregated spend.
- **application_logs / traces / metrics_snapshots** — logging, tracing, and metrics.
- **retrieval_logs** — retrieval operations (filter_metadata, retrieved_chunks,
  similarity_scores, retrieval_time_ms).
- **audit_logs** — sensitive actions (user_id, action, resource, resource_id). Actions
  include LOGIN, UPLOAD_DOCUMENT, REINDEX, ACTIVATE_VERSION, UPDATE_ROLE, etc.

## Relationships

- users 1—N sessions; users 1—N conversations.
- documents 1—N document_versions 1—N chunks (each chunk holds its embedding).
- conversations 1—N questions 1—N answers.
- answers 1—N citations N—1 chunks.
- questions 1—N retrieval_logs, llm_usage, feedback.
- audit_logs, application_logs reference users/actions.

## Validation Rules (derived from requirements)

- Every chunk MUST have populated metadata (document, category, version, status, page,
  section, effective date) — supports constitution Principle II.
- Exactly one ACTIVE document_version per document family — supports Principle III.
- Only ACTIVE versions participate in retrieval (later phases) — enforced via status
  field present in schema.
- Embeddings MUST be stored in vector column (dimension 1024) — supports FR-003.

## Indexes & Constraints

- Unique email on users.
- Index on documents(category), document_versions(status), questions(predicted_category).
- GIN full-text index on chunks(content) for hybrid retrieval (later phase).
- IVFFlat vector index on chunks(embedding) using vector_cosine_ops.

## Migration Strategy

- Alembic migrations create all tables, indexes, and vector extension.
- FR-008: migrate a fresh environment cleanly (all objects created); subsequent
  migrations apply only new changes; history tracked in repository.

**Note**: Full SQL DDL is specified in docs/DATA_MODEL.md (Section 12-13). The Phase 0
migrations realize that schema; no new fields are introduced beyond the documented model.
