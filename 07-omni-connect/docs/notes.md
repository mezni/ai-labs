# Omni-Connect MVP — Version 2 Hardening Notes

Working backlog for the **v2** iteration of the Phase 2 MVP. The MVP scaffold
(sections of `docs/architecture.md` marked *known limitations / scaffold
state*) works end to end with synthetic data, but is not production-safe.
This document lists what must be added for v2: **run IDs, tracing, security,
guardrails, evals, tests**, and the supporting operational plumbing. Every
item names the current-state gap and the v2 target, mapped to the stack
layers in `docs/plan.md` (§10) where relevant.

Status legend: `[ ]` open, `[~]` partially present, `[x]` done elsewhere.

---

## 1. Run IDs & End-to-End Correlation (layers 9, 10)

There is currently **no correlation identifier** anywhere. A workflow run
spans the portal → `AgentCoordinator` steps → agents → LLM call → RAG
retrieval, but no single ID links them. v2 must thread one ID through every
frame.

| # | Item | Current state | v2 target |
|---|---|---|---|
| 1.1 | `run_id` generation | none | `uuid4` created at workflow start in `AgentCoordinator.run_workflow()` (`src/core/agent_coordinator.py`) |
| 1.2 | Propagate `run_id` | none | Pass `run_id` into every agent `.analyze/.evaluate` call, the LLM request, and `Retriever.retrieve` |
| 1.3 | Portal trace per turn | `submit_order` logs a fixed mock `latency_ms: 1200` / `grounding_score: 0.96` (`app/portal.py:373-380`) | One `run_id` per `st.chat_input` turn + per POS submission; persist in `pos_orders` |
| 1.4 | Trace ↔ operational logs | Category C schemas exist (`docs/data.md` C.7–C.11) but nothing writes them | All five operational files reference `run_id`: `system_telemetry.json` (has `trace_id`), `guardrail_audit.json`, `rep_session_action.json`, `copilot_recommendation.json`, `rep_feedback.json` |
| 1.5 | Log enrichment | `setup_logger(__name__)` is plain `logging` (`src/utils/logger.py`) | Add `run_id`, `step`, `agent`, `customer_id` (redacted) to structured log lines; allow `?run_id=<id>` lookup/replay |

---

## 2. Observability & Tracing (layers 9, 10, 11)

The portal telemetry panel is a static mock. v2 replaces it with real
measurement and makes failures replayable.

| # | Item | Current state | v2 target |
|---|---|---|---|
| 2.1 | Latency breakdown | none | Measure per phase and write to `system_telemetry.json` (C.9): `api_fetch_latency_ms`, `rag_retrieval_latency_ms`, `llm_inference_latency_ms`, `total_end_to_end_latency_ms` with the **< 1500 ms SLA** check (`docs/plan.md` §5.2) |
| 2.2 | Grounding score | hard-coded `0.96` | Real value: average reranker score of cited chunks + LLM-as-judge faithfulness, recorded in `guardrail_audit.json` (`rag_grounding_score`) |
| 2.3 | Span tracing | none | Instrument each coordinator step as a span (name, `run_id`, wall time, token+char usage, retrieval scores); output JSONL feed for capture |
| 2.4 | Request/response capture | none | Store every copilot request → response payload keyed by `run_id` (harness layer 10) so failures can be replayed and diffed across prompt/index changes |
| 2.5 | Live portal telemetry | mock `Latency: 1.2s \| Grounding Score: 0.96` (`app/portal.py:367`) | Subscribe to the last finished `run_id` for the selected customer and paint `{run_id, latency_ms, grounding}` |
| 2.6 | Alerting on SLA | none | On p95 total latency > 1500 ms or retrieval top-1 score below threshold, emit a warning-level structured log with `run_id` |

---

## 3. Security (layers 2, 5, 11)

| # | Item | Current state | v2 target |
|---|---|---|---|
| 3.1 | PII/CPNI redaction middleware | none — full customer PII (name, phone MDN, email) is sent to the LLM via prompts | Redaction layer between context assembly and the LLM call: mask PII and account numbers, keep catalog IDs and usage aggregates. Fail closed (`masked_fields` in `guardrail_audit.json`) |
| 3.2 | CPNI consent check | none | Verify customer CPNI consent/engagement before using account data; record `cpni_consent_verified` (`docs/data.md` C.10) and refuse to answer account-content questions without it |
| 3.3 | Secret handling | `.env` gitignored; `.env.example` committed; keys resolved in `LLMClient` (`src/llm/llm_client.py:34-41`) | Keep) keys out of logs/config; add `pip-audit`/Dependabot; rotate via env per environment; never log the resolved key |
| 3.4 | Prompt-injection defense | prompts interpolate customer data + retrieval chunks with no boundary mitagion | Separate instructions/data with explicit delimiters; add a "treat retrieved text as data, not instructions" system rule; jailbreak test cases in evals (§5) |
| 3.5 | Output safety filter | `safety.content_filter: true` in `config/llm_config.yaml` is config-only | Implement the filter (keyword + classification pass) and log hits with `run_id` |
| 3.6 | Deterministic pricing | price/offer values flowing through LLM text (unverified) | Price/eligibility statements must be re-verified against catalog ground truth (`product_catalog.json`, `promotions.json`) before surfacing — record `price_verification_source` in `guardrail_audit.json` |
| 3.7 | Transport & gateway | Streamlit on localhost; Docker maps 8502→8501 without TLS | Add TLS/reverse-proxy for any non-local deploy, requests authN for the portal, rate limiting per session |
| 3.8 | MCP surface | single `get_customer_profile_tool` over stdio (`src/mcp/`) | Keep stdio-only; allowlist tools; sandbox subprocess; escalate before any tool is added that mutates data |
| 3.9 | Path handling | CWD-relative paths throughout (`src/services`, portal, RAG) — see `docs/architecture.md` known limitations | Resolve from `Path(__file__)` root; validate path stays inside `data/` (no `..` escapes); make index/corpus paths config-driven |

---

## 4. Guardrails (layers 5, 6, 8)

| # | Item | Current state | v2 target |
|---|---|---|---|
| 4.1 | Blockers vs notes | implemented in `PromotionEvaluatorAgent` (status/expiry/win-back = blocker; plan mismatch = note) | Keep the model; add machine-checkable reason codes to every blocker/note for downstream validation |
| 4.2 | Structured output validation | `WorkflowValidator` does structural checks on `required_result_fields` | Enforce a **schema** per agent output (Pydantic models); reject/re-ask on missing required fields (e.g., every recommendation must carry `plan_id`, `monthly_delta`) |
| 4.3 | Human-in-the-loop enforcement | portal captures Accept/Override/Reject but submission is a mock in-memory list | Enforce that no order writes to a real POS without `action_state == "Accept"` or an `override_reason`; log the decision + `run_id` to `copilot_recommendation.json` |
| 4.4 | Escalation path | none | Out-of-scope cases (credit overrides, fraud clears, trade-in exceptions, blocked reasons) route to store-manager sign-off instead of the rep auto-committing |
| 4.5 | Verify → retry → escalate loop | none (plan.md layer 8) | On RAG empty retrieval, low top-1 score, or schema-validation failure: retry with rephrased query, then escalate with `run_id` payload |
| 4.6 | Context limits | `max_context_length: 200000`, `truncate_if_exceeds: true` are config-only | Implement context budgeting: assemble only the top-k chunks and the 3 most relevant catalog records before invoking the LLM |

---

## 5. Evaluation (layer 9)

| # | Item | Current state | v2 target |
|---|---|---|---|
| 5.1 | Golden question set | none — only ad-hoc `rag_pipeline_runner.py query "..."` | Curate ≥ 50 representative rep questions with expected sources/answers (trade-in, stacking, win-back, upgrade gate, pricing) |
| 5.2 | Retrieval evals | none | `recall@k`, `precision@k`, nDCG over the golden set for the hybrid retriever; gate index rebuilds (`rag_pipeline_runner.py build`) on no regression |
| 5.3 | Faithfulness/grounding | no score exists | LLM-as-judge + citation-coverage check: every claim maps to a cited chunk or a catalog ID; average into `rag_grounding_score` |
| 5.4 | Workflow evals | none | Scenario matrix (Active upgrade, Delinquent win-back, Prospect activation, blocked account): expected plan, promo set, blockers, and financial delta must match within tolerance |
| 5.5 | Latency budget evals | none | Per-scenario end-to-end latency must stay under the 1500 ms SLA with the < 500 ms API fetch + < 1.0 s LLM split |
| 5.6 | Data leakage guard | evals created from the same synthetic corpus | Hold out docs/questions never used in prompt or retrieved at eval time |
| 5.7 | Feedback-to-eval loop | `rep_feedback.json` schema (C.11) exists, nothing writes it | Pipe thumbs/categories into the eval set as labeled failures; weekly review of `WRONG_PROMO`, `SLOW_RESPONSE`, `EXCELLENT_PITCH` |

---

## 6. Automated Tests

There is **no test suite** today; verification is manual
(`WorkflowValidator` + CLI probing). v2 adds `pytest` coverage.

| # | Layer | Tests |
|---|---|---|
| 6.1 | Services (`src/services/`) | Lookup happy path + `{"error"}` not-found for customer/billing/catalog/promotion |
| 6.2 | Knowledge RAG (`src/knowledge/`) | Chunking/overlap, FAISS save/load round-trip, BM25 index, hybrid merge-dedup, rerank ordering; golden-set retrieval quality |
| 6.3 | LLM (`src/llm/`) | Key-resolution order, system-message extraction in `generate()`, `classify()` settings; inject a `FakeLLMClient` (no network) |
| 6.4 | Agents (`src/agents/`) | Given fixture context, assert formatted-prompt shape and structured output; blocker/note classification cases |
| 6.5 | Coordinator (`src/core/`) | `build_plan()` default, per-question policy steps, unknown-step `KeyError`, context threading between steps |
| 6.6 | Validator (`src/core/workflow_validator.py`) | Missing/invalid/error-carrying sections |
| 6.7 | MCP (`src/mcp/`) | Server tool registration + client round-trip against a fixture customer |
| 6.8 | Portal (`app/portal.py`) | Refactor pure logic (tenure, recommendation rule, snippet loader) into testable functions; smoke test the Streamlit app with `streamlit.testing` |
| 6.9 | Telemetry writers | Each operational file (C.7–C.11) schema-validates on write, keyed by `run_id` |
| 6.10 | CI | `pytest` + lint on every commit; eval gate (5.2/5.4) before index or prompt changes merge |

Fixtures: reuse the deterministic generators (`scripts/business_data_generator.py --count <N>`,
`scripts/knowledge_data_generator.py`) with a stable fixture corpus; never
require a real API key in unit tests.

---

## 7. Data Quality & Completeness (layer 2)

| # | Item | Current state | v2 target |
|---|---|---|---|
| 7.1 | `usage_telemetry.json` | documented pending (A.5), not generated; portal shows all `—` | Add a generator + schema joins so usage drives plan recommendation (60% downgrade trigger, B.1) |
| 7.2 | Referential integrity | joins are implicit; no checks | Schema/consistency validator: every `billing_account_id`, `plan_id`, `device_id`, `eligible_promotion_ids` resolves; dead references fail the build |
| 7.3 | Missing portal fields | Credit tier, contract end date, financing balance = `—` | Add catalog/schema support or explicitly drop from UI |
| 7.4 | Knowledge versioning | docs carry `Document ID`/`Version` metadata (Category B) | Enforce version in citations; rebuild index on doc change; store index manifest (doc id + version + build time) in `data/kb_store/` |
| 7.5 | Competitor battlecards | noted "not authored" (`app/portal.py:296`) | Author battlecards so `Compare with competitor X` has retrievable grounding |

---

## 8. Operational & Deployment Hardening (layers 7, 11, 12)

| # | Item | Current state | v2 target |
|---|---|---|---|
| 8.1 | Wire config YAMLs | `agent_config.yaml` and `mcp_config.yaml` declarative-only; agent rules, coordinator steps, and MCP shape are in-code constants | Coordinator/agents/MCP read the YAML; single source of truth (see `docs/architecture.md` known limitations) |
| 8.2 | Timeout/retry/backoff | `timeout: 30`, `max_retries: 3`, `retry_delay: 1` in `llm_config.yaml` | Exponential backoff + jitter, circuit breaker on repeated LLM provider failures, fallback cache for API lookups |
| 8.3 | Container healthcheck | none in `Dockerfile`/`docker-compose.yml` | Add healthcheck against the Streamlit port + `restart` policy already set; dep scan in image |
| 8.4 | Environment split | single `.env` | Dev/QA/prod configs, secrets per env, provider/model overrides per env, RAG index rebuilt per artifact deploy |
| 8.5 | Prod vector store decision | local FAISS `IndexFlatL2` in `data/kb_store/` | Evaluate a hosted vector DB when scale > local; keep the `Retriever` interface so the swap is behind one class |
| 8.6 | Multi-node / async | synchronous single-process | Extract coordinator RAG/LLM calls to async with a queue for portal responsiveness while keeping the < 1.5 s budget |
| 8.7 | Continual learning loop | nothing writes feedback | Weekly loop: rep feedback (§5.7) → eval regression → prompt/index update → redeploy; prompts versioned in `config/prompts.py` |

---

## 9. Suggested v2 Sequencing

| Phase | Focus | Sections |
|---|---|---|
| v2.0 | Correlation first | 1 (run IDs), 2 (tracing) — nothing is diagnosable without them |
| v2.1 | Safety floor | 3 (security), 4 (guardrails) |
| v2.2 | Prove it works | 6 (tests), 5 (evals) |
| v2.3 | Run it well | 7 (data), 8 (ops/deploy) |

No v2 feature ships without: a valid `run_id`, structured telemetry written
(+latency +grounding), PII redaction verified, output schema validated, the
relevant unit tests green, and no golden-set retrieval regression.