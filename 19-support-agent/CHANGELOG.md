# Changelog

## Version History

| Version | Feature Domain | Key Objectives |
| --- | --- | --- |
| 0.0.13 | Explicit Support Ticket Workflow | Added explicit support ticket workflow. |
| 0.0.12 | Harden Agent Loop Failure Handling | Hardened agent loop failure handling. |
| 0.0.11 | Multi-turn Conversation Memory | Added persistent agent sessions that retain conversation history across turns. |
| 0.0.10 | Bounded Agent Loop | Added 5-round retry loop with tool execution and error handling for reliable LLM agent operations. |
| 0.0.9 | Tool Calling | Added handle_ticket tool for LLM-mediated ticket classification. |
| 0.0.8 | Improve LLM Client Reliability | Improve LLM client reliability. |
| 0.0.7 | Prompt Engineering and Evaluation | Prompt engineering and evaluation implemented. |
| 0.0.6 | Structured LLM Outputs | Enforce structured LLM outputs. |
| 0.0.5 | LLM Ticket Classification | Integrate LLM ticket classification. |
| 0.0.4 | Edge Cases, Regression Tests, and Logging | Edge cases, regression tests, and logging implemented. |
| 0.0.3 | Data Contracts | Implement TicketInput and TicketOutput. Integrated validation into the classifier. Added schema tests and updated classifier tests. |
| 0.0.2 | Business Logic | Implement the ticket classifier. |
| 0.0.1 | Project Foundation | Initialized uv project (pyproject.toml, uv.lock, virtualenv) with a src layout. Added runtime dependencies: anthropic, pydantic, python-dotenv. Added development dependency: pytest. Added .env.example and gitignored local .env. Scaffolded package layout under src/support_agent/ and tests/. Added sample datasets data/knowledge_base.json and data/tickets/tickets.json. |

## [0.0.13] — Explicit Support Ticket Workflow
- Added explicit support ticket workflow.

## [0.0.12] — Harden Agent Loop Failure Handling
- Hardened agent loop failure handling.

## [0.0.11] — Multi-turn Conversation Memory
- Added persistent agent sessions that retain conversation history across turns.

## [0.0.10] — Bounded Agent Loop
- Added 5-round retry loop with tool execution and error handling for reliable LLM agent operations.

## [0.0.9] — Tool Calling
- Added handle_ticket tool for LLM-mediated ticket classification.

## [0.0.8] — Improve LLM Client Reliability
- Improve LLM client reliability.

## [0.0.7] — Prompt Engineering and Evaluation
- Prompt engineering and evaluation implemented.

## [0.0.6] — Structured LLM Outputs
- Enforced structured LLM outputs.

## [0.0.5] — LLM Ticket Classification
- Integrated LLM ticket classification.

## [0.0.4]
- Edge cases, regression tests, and logging implemented.

## [0.0.3] — Data Contracts
- Implemented TicketInput and TicketOutput.
- Integrated validation into the classifier.
- Added schema tests and updated classifier tests.

## [0.0.2] — Business Logic
- Implement the ticket classifier

## [0.0.1] — Foundation
Initialized uv project (pyproject.toml, uv.lock, virtualenv) with a src layout.
Added runtime dependencies: anthropic, pydantic, python-dotenv.
Added development dependency: pytest.
Added .env.example and gitignored local .env.
Scaffolded package layout under src/support_agent/ and tests/.
Added sample datasets data/knowledge_base.json and data/tickets/tickets.json.