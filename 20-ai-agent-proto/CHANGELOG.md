# Changelog

All notable changes, problem-solving decisions, and iterations for this AI agent project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- [Feature/Step]: Description of the new agent feature, tool, or memory module.

### Changed
- [Modification]: Adjustments to prompts, temperature settings, or tool definitions.

### Fixed & Resolved (Problem -> Solution Log)
- **Issue:** Support ticket classification — plain text tickets must be classified into structured results
  - **Goal:** Extract category, order ID, and lookup needs from support tickets
  - **Solution:** Implemented rule-based ticket classifier with regex order ID extraction and keyword-based categorization
- **Issue:** Module import failure — `ai-agent` package name conflict prevented `from ai_agent.runner import run_pipeline`
  - **Goal:** Resolve ModuleNotFoundError and enable `uv run python main.py` to work
  - **Solution:** Renamed `src/ai-agent` → `src/ai_agent`, updated `pyproject.toml` name field, and fixed Hatchling package discovery config
- **Issue:** Incoming payloads contained unexpected keys, missing required fields, or invalid empty string values, causing unhandled runtime exceptions
  - **Goal:** Ensure raw incoming payload dictionaries are reliably validated, sanitized, and typed before entering the agent's execution pipeline
  - **Solution:** Introduced `RawTicketInput` using Pydantic with `extra="ignore"` to strip unexpected keys, added custom field validators to reject empty/whitespace-only input strings, and implemented unit test coverage (`test_contract.py`) for schema validation failures
- **Issue:** Rule-based keyword checks break down with ambiguous inputs, missing IDs, or overlapping categories
  - **Goal:** Turn the ticket classifier into a production-ready, maintainable component with deterministic behavior
  - **Solution:** Enhanced classification logic with explicit keyword priority (refund/dispute > order lookup), confidence scoring (0.60–0.98), operational flags (`needs_human_review`, `needs_order_lookup`), and audit assumptions list; updated data contracts (`src/ai_agent/models.py`) and classification logic (`src/ai_agent/classifier.py`); comprehensive test suite (`tests/test_classifier.py` + `tests/test_contract.py`)
  - **Trade-off:** Expanded rule-based logic with explicit precedence vs. ML classifier — chosen for deterministic behavior, clear audit trails, and immediate implementability without training data
- **Issue:** Operational metadata now captures text length, word count, and regex match status instead of raw customer text
  - **Goal:** Privacy-compliant telemetry that preserves structural metrics while excluding PII
  - **Solution:** Introduced `OperationalMetadata` model (`text_length`, `word_count`, `has_order_id_pattern`); classifier constructs metadata from validated text and logs only structural metrics; `test_rule_5_operational_metadata_logging` verifies no raw message leakage
  - **Trade-off:** Added intermediate model layer vs. flat output — chosen for privacy compliance and separation of concerns
- **Issue:** Transition from keyword heuristics to LLM structured classification
  - **Goal:** Interpret semantic intent across diverse customer query phrasings while returning a structured schema
  - **Solution:** Integrated gpt-4o-mini using OpenAI's structured output interface (`response_format=LLMTicketClassification`). Enforced type-safe Pydantic response parsing for category, confidence, extracted order_id, and step-by-step reasoning via `LLMTicketClassification` model and `classify_ticket_with_llm()` engine; integration test suite (`tests/test_llm_classifier.py`) validates semantic variations
  - **Trade-off:** API dependency and cost vs. improved semantic coverage — chosen for handling implicit/idiomatic customer phrasings that hard-coded rules miss

---

## [0.1.0] - 2026-10-11

### Added
- Project initialization with `uv` for dependency management, `ruff` for code quality, and `pytest` for test suites.
- Initial setup of core AI agent loop with structured system prompts.
- Schema validation for tool execution layer.

### Fixed & Resolved
- **Issue:** Hallucinating function parameters during tool execution.
  - **Goal:** Strict schema adherence for external tool calls.
  - **Solution:** Integrated Pydantic validation models at the tool call entry point and added 2-shot examples to the system prompt. Tested using `pytest`.