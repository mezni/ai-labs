# Omni-Connect — Missing Documentation Backlog

Catalog of documentation that does not exist yet, grouped by audience and
priority. Status legend: `[ ]` not started, `[~]` partially covered elsewhere,
`[x]` exists.

Current docs inventory: `brief.md` (business), `architecture.md`, `data.md`
(schemas), `plan.md` (Phase 2 MVP), `portal.md` (UI reference), `notes.md`
(v2 hardening backlog).

---

## 1. Core user-facing & reference docs

| # | Doc | Status | What it should contain | Priority |
|---|---|---|---|---|
| 1.1 | `README.md` | `[ ]` | Top-level entry point: what the project is, quickstart (`.env`, data generators, RAG build), run commands, docs index | High |
| 1.2 | `docs/configuration.md` | `[ ]` | Reference for `config/llm_config.yaml`, `config/agent_config.yaml`, `config/mcp_config.yaml`, `config/prompts.py`, and `.env` vars (`ANTHROPIC_API_KEY`, `LLM_BASE_URL`, `HF_TOKEN`). Note that `agent_config.yaml` / `mcp_config.yaml` are declarative-only today | High |
| 1.3 | `docs/api.md` | `[ ]` | Module/API reference for `src/services/*`, `src/knowledge/*`, `src/llm/*`, `src/core/*`, `src/mcp/*` — signatures, return shapes (`{"error": ...}` convention), usage examples | High |
| 1.4 | `docs/knowledge-base.md` | `[ ]` | How to author/add policy docs: required metadata (`Document ID`, `Version`, `Department`, `Overview`), chunking rules (1000-char, 200 overlap), corpus glob (`*.md.txt`), index rebuild via `scripts/rag_pipeline_runner.py build` | High |
| 1.5 | `docs/prompts.md` | `[ ]` | `config/prompts.py` template guide: available prompts, template variables, grounding/masking expectations, blocker-vs-notes convention (see `docs/plan.md` §10 layer 6) | Medium |
| 1.6 | `docs/mcp.md` | `[ ]` | `src/mcp/` protocol surface: tools, stdio transport, `mcp<2` pin rationale, client/server usage, notebook-safe wrapper | Medium |
| 1.7 | `docs/operate.md` | `[~]` | Operational runbook beyond the short "Operate" section of `architecture.md`: docker-compose ops, index lifecycle, log locations, troubleshooting checklist, SLA monitoring | Medium |

## 2. Engineering process docs

| # | Doc | Status | What it should contain | Priority |
|---|---|---|---|---|
| 2.1 | `docs/decisions/` (ADRs) | `[ ]` | Decision records for: local RAG FAISS vs hosted vector store, Anthropic vs OpenRouter providers, deterministic `build_plan()` vs PlannerAgent, `mcp<2` pin, errors-as-data convention, `data/kb_store` relocation | High |
| 2.2 | `docs/testing.md` | `[ ]` | Test design once the pytest suite lands (see `notes.md` §6): fixtures via deterministic generators, fake LLM injection, CI wiring, eval gates | Medium |
| 2.3 | `docs/security.md` / `docs/compliance.md` | `[ ]` | CPNI/PII redaction, prompt-injection defense, deterministic pricing, access control — currently only backlog items in `notes.md` §3–4 | Medium |
| 2.4 | `docs/roadmap.md` | `[ ]` | Phase 3/4 spec (Customer AI Assistant, omnichannel) referenced by `plan.md` but not defined anywhere | Medium |
| 2.5 | `CHANGELOG.md` | `[ ]` | Release history aligned to git commits | Low |
| 2.6 | `CONTRIBUTING.md` | `[ ]` | Repo conventions: run from repo root (CWD-relative paths), dual manifests (`pyproject.toml` + `requirements.txt`) in sync, docs update policy | Low |

## 3. Partially covered / folded into other work

| Doc | Status | Where it currently lives | Action |
|---|---|---|---|
| `docs/usage.md` / telemetry + evals state | `[~]` | Backlog items in `notes.md` §2 (tracing), §5 (evals), §7 (data) — lives in `docs/data.md` C.7–C.11 schemas | Promote to a real doc once observed |