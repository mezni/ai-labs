<!--
Sync Impact Report

  Version change: (unratified template scaffold) -> 1.0.0 (initial ratification)

  Modified principles: none (no previously ratified principles existed)

  Added sections:
    - Core Principles I.-V.
    - Security & Trust Requirements
    - Delivery, Evaluation & Deployment
    - Governance (amendment procedure, versioning policy, compliance review)

  Removed sections: none (template placeholder comments replaced with concrete content)

  Follow-up TODOs: none
-->

# Telecom Policy Assistant Constitution

## Core Principles

### I. Retrieval Before Generation (NON-NEGOTIABLE)

The assistant MUST NOT answer a policy question without retrieved evidence from the
knowledge repository.

- Every answer MUST be generated only from retrieved policy content (grounding
  enforcement), never from parametric knowledge alone.
- When no evidence exists or confidence is low (< 0.70), the assistant MUST refuse with
  the standard fallback: "I could not find this information in the available policy
  documents."
- The assistant MUST NOT invent fees, charges, contract terms, legal conditions,
  penalties, or policy rules.
- Restricted topics (other customers' data, unpublished policies, future promotions,
  pricing not in policy documents) MUST be refused.

Rationale: Grounded refusal prevents hallucination, protects compliance, and preserves
user trust in answer correctness.

### II. Source Grounding & Citation Traceability

Every claim MUST be traceable to its source document.

- Every answer MUST include citations (document name, page number, section) for each
  claim; citation coverage MUST be 100% and citation accuracy MUST exceed 95%.
- Every question -> retrieved chunks -> answer -> citations MUST be auditable
  end-to-end (audit logs, retrieval logs, traces).
- Chunks MUST represent complete policy concepts (rules, conditions, and exceptions
  kept together; sections never mixed).
- Metadata (document, category, version, status, page, section, effective date) MUST be
  populated and validated on every chunk.

Rationale: Auditability and explainability are legal and operational requirements for a
policy-asset platform (DATA_MODEL DM-001, DM-003).

### III. Active Knowledge Governance

Retrieval MUST operate only on approved, current policy content.

- Metadata filtering (category, status, version, effective date) MUST precede semantic
  search whenever possible.
- Only document versions with status ACTIVE MAY be retrieved; exactly one ACTIVE
  version per policy family.
- Documents flagged as >= 95% duplicate MUST be rejected, replaced, or versioned, not
  silently duplicated.
- The assistant answers exclusively from approved policy documents - it MUST NOT draw on
  customer account data, billing engines, or out-of-scope sources.
- New document types MUST be ingestible without code changes.

Rationale: Retired or draft policies must never surface as authoritative answers, and
duplicate or mixed content degrades retrieval precision (RET-003, RET-004).

### IV. Observability by Design

Every request MUST be observable and traceable.

- Structured JSON logging, distributed tracing (Langfuse), and Prometheus metrics MUST
  cover every request (question, retrieved sources, generated answer, model, latency).
- Availability SLO MUST be >= 99.5%; P95 API latency MUST be < 5 seconds; retrieval
  success rate MUST be > 99%.
- Performance targets MUST be met: retrieval < 500 ms, re-ranking < 1000 ms, total
  retrieval pipeline < 2 seconds.
- Guardrails, fallbacks, injection attempts, and negative feedback MUST be logged and
  reviewed on a defined cadence.

Rationale: Traceability enables rapid diagnosis, security investigation, and continuous
quality improvement (AP-004).

### V. FinOps by Design

Every AI interaction MUST be measurable and attributable.

- Every model invocation MUST record user, store, model, question, tokens, latency, and
  estimated cost.
- Cost per question MUST remain below $0.03; average tokens per request MUST stay below
  3000.
- Budget thresholds MUST trigger action: 80% warning (notify platform team), 90% high
  (notify product owner), 100% critical (escalate and review usage).
- Cost-optimization initiatives (caching, context reduction, prompt compression, model
  routing) MUST NOT degrade answer quality.

Rationale: AI spend must be governed like any business cost, with visibility,
attribution, and budget enforcement (FIN-001 through FIN-005, AP-005).

## Security & Trust Requirements

The platform MUST meet the controls defined in SECURITY.md.

- **Authentication**: OIDC with Microsoft Entra ID / Keycloak; every endpoint requires
  authentication unless explicitly documented.
- **Authorization**: RBAC with RETAIL_AGENT, SUPERVISOR, ADMIN, SYSTEM roles; least
  privilege MUST be enforced; rate limits 60 / 120 / 300 requests per minute by role.
- **Data protection**: encryption at rest (AES-256) and in transit (TLS 1.2+);
  encrypted, immutable, access-controlled backups; retention per corporate policy.
- **Secrets**: MUST NOT appear in source code or git; stored in HashiCorp Vault or Azure
  Key Vault only.
- **AI guardrails**: prompt injection success rate MUST be 0%, data leakage events MUST
  be 0%, PII retention MUST be 0, unbacked claims MUST be refused; prompts MUST be
  versioned and stored outside application code.
- **Delivery security**: every release MUST pass dependency, SAST, container, and secret
  scans; no critical vulnerabilities may deploy; patch SLAs are 7 / 30 / 90 days for
  critical / high / medium.

## Delivery, Evaluation & Deployment

Work MUST proceed through the phased execution plan and quality gates defined in
EXECUTION_PLAN.md and RUNBOOK.md.

- **Phases**: Foundation -> Ingestion -> Retrieval -> Generation/API/Guardrails ->
  Observability/FinOps -> Evaluation -> CI/CD/Go-Live.
- **Release gates**: before production, ALL MUST pass - Recall@5 >= 90%, citation
  accuracy >= 95%, answer accuracy >= 90%, hallucination rate < 2%, latency < 5 seconds,
  cost targets met, unit and integration tests passed, security scan passed, no critical
  vulnerabilities.
- **Environment promotion**: Development -> Test -> Staging -> Production, each stage
  with documented approvals (CI checks, engineering lead, product owner).
- **Regression evaluation** MUST run after any change to prompts, embedding model,
  reranker, retrieval logic, chunking strategy, LLM provider, or metadata rules.
- **Rollback** MUST be tested and functional; blue-green or rolling deployment with
  automated health checks.
- Adversarial red-team testing MUST run quarterly and after every prompt or model change.

Rationale: Released increments must be measured, secure, and reversible so confidence and
compliance scale with adoption.

## Governance

This constitution supersedes all other project development practices. Detailed runtime
guidance lives in the reference documents under docs/ (PRD, ARCHITECTURE, DATA_MODEL,
INGESTION_SPEC, RETRIEVAL_STRATEGY, API_SPEC, SECURITY, OBSERVABILITY, FINOPS,
EVALUATION_PLAN, RUNBOOK).

- Amendments require a written proposal, updated sections in this document, an approved
  migration/communication plan, and a semantic version bump:
  - MAJOR: backward-incompatible removal or redefinition of a principle.
  - MINOR: a new principle or materially expanded guidance.
  - PATCH: clarification, wording, or non-semantic refinement.
- Every PR and review MUST verify compliance with these principles and the applicable
  quality gates before merge.
- Complexity and AI cost MUST be justified and measurable in each review; nothing is
  exempt from Observability by Design or FinOps by Design.
- Evaluation, security, and cost KPIs MUST be reviewed continuously in production.

**Version**: 1.0.0 | **Ratified**: 2026-09-08 | **Last Amended**: 2026-09-08