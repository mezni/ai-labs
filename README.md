# AI Experiments

A workspace of incremental AI experiments — each directory is a standalone, self-contained project. The emphasis is on understanding every component from first principles before reaching for frameworks.

## Projects

| Dir               | Status      | Description                                                                                                  |
|--------------------|-------------|--------------------------------------------------------------------------------------------------------------|
| `01-rag-naive`     | Implemented | Minimal RAG indexing & retrieval pipeline (FAISS + OpenRouter + MiniMax, pure Python, `rag-indexer` CLI)      |
| `02-rag-dataops`   | Implemented | RAG DataOps pipeline (LlamaIndex + ChromaDB + OpenRouter, versioned indexes, lineage, rollback, `rag-dataops` CLI + Streamlit dashboard) |
| `03-rag-system`    | Working v0  | Aether Wireless policy-search RAG system — three pipelines (ingestion / retrieval / evaluation), Postgres + pgvector, role-play from business brief |
| `04-rag-project`   | In progress | Second-generation "rag-project" for Aether Wireless — domain model, document lifecycle & chunk lineage, staged ingestion, pgvector persistence, FastAPI + Streamlit |
| `06-ragops`        | Implemented | Production-oriented RAG pipeline (ChromaDB, stage-based architecture, 88+ tests); `version0/` is the original LlamaIndex/pgvector scaffold |
| `07-omni-connect`  | MVP in progress | AI-Powered Telecom Retail Copilot — unified omnichannel copilot platform (agents, MCP, RAG, Streamlit portal) for telecom retail |

## Description

The projects build on each other, each one hardening the RAG story a step further:

**RAG Indexing & Retrieval Pipeline** in [`01-rag-naive/`](01-rag-naive/):
A lightweight, local-first Retrieval-Augmented Generation (RAG) system built in pure Python — no RAG framework. It ingests PDFs, detects new/changed/unchanged/deleted documents by SHA-256, chunks and embeds content, stores vectors in FAISS with explicit IDs and JSON metadata, and answers questions through a MiniMax generation model with source citations, all routed through OpenRouter. A `rag-indexer` CLI exposes `index`, `reindex`, `search`, and `ask`; sample policy PDFs are generated on demand by an LLM-driven script.

**RAG DataOps Pipeline** in [`02-rag-dataops/`](02-rag-dataops/):
A DataOps-oriented RAG pipeline with LlamaIndex + ChromaDB: heterogeneous document ingestion (PDF, DOCX, HTML, TXT, SQLite), deterministic SHA-256 change detection, document versioning and lineage, versioned vector snapshots with full reproducibility and physical rollback. CLI (`rag-dataops index/versions/rollback/catalog/search`) + multipage Streamlit dashboard.

**Aether Wireless Policy Search** in [`03-rag-system/`](03-rag-system/):
A role-play RAG system for a fictional telecom carrier with living policy documents. Three independent pipelines share one Postgres + pgvector database: ingestion (multi-source, versioned with hash-based change detection), retrieval (grounded, cited answers for support agents), and evaluation (golden-question Recall@k / MRR gates in CI). Guardrails, run tracking, and prompt versioning are cross-cutting. Build progress is tracked in [`docs/PROGRESS.md`](03-rag-system/docs/PROGRESS.md).

**rag-project** in [`04-rag-project/`](04-rag-project/):
The second-generation rework of the Aether Wireless system. Adds a dedicated domain-entity layer (document, chunk), document lifecycle and chunk-lineage design docs, stage-based ingestion (discover → parse → clean → chunk → enrich → embed → persist) on LlamaIndex, persistence on PostgreSQL/pgvector via a SQLAlchemy repository layer with Alembic migrations, retrieval with rank-based evals and LLM-as-judge grounding, plus FastAPI and Streamlit UIs. Changelog tracked in [`CHANGELOG.md`](04-rag-project/CHANGELOG.md).

**RAG Ops** in [`06-ragops/`](06-ragops/):
A production-oriented RAG pipeline for Aether Wireless (fiber/broadband/mobile rate cards and spec sheets). Stage-based ingestion split into single-purpose stages, real embedding models via OpenRouter, ChromaDB persistence, recursive structure-aware chunking, pdfplumber table extraction with OCR fallback, document hashing + idempotent dedup, versioned documents with rollback protection, centralized config (`ragops.toml` + env), and a 88+ test suite. `version0/` is the earlier LlamaIndex/pgvector scaffold. Ingestion evolution is logged in [`docs/PROGRESS.md`](06-ragops/docs/PROGRESS.md).

**Omni-Connect** in [`07-omni-connect/`](07-omni-connect/):
AI-Powered Telecom Retail Transformation — a unified AI Retail Copilot platform that empowers retail representatives and customers with real-time intelligence, personalized recommendations, and seamless omnichannel experiences. Agent orchestration, MCP tools, knowledge/RAG pipelines, and a Streamlit portal, with v2 hardening (run IDs, tracing, security, guardrails, evals) tracked in [`docs/notes.md`](07-omni-connect/docs/notes.md).

## Tech Stack

| Layer         | Technology                                                                                     |
|---------------|------------------------------------------------------------------------------------------------|
| Language      | Python 3.12+                                                                                   |
| Packaging     | UV                                                                                             |
| Vector stores | FAISS CPU (`faiss-cpu`), ChromaDB (`chromadb`), Postgres + pgvector                             |
| Document parsing | PyPDF, pdfplumber, SimpleDirectoryReader (LlamaIndex), MarkdownNodeParser, OCR (OCR via `pdf2image` + `pytesseract`) |
| Frameworks    | LlamaIndex, Streamlit, FastAPI, SQLAlchemy + Alembic                                            |
| Model access  | OpenRouter (OpenAI-compatible client), MiniMax (`minimax/minimax-m3`), HuggingFaceEmbedding (`BAAI/bge-small-en-v1.5`) |
| Config        | Pydantic + `pydantic-settings`, `python-dotenv`, TOML config (`ragops.toml`)                    |
| Testing       | pytest (unit + integration + e2e, stubbed embeddings for offline runs)                          |
| CI/CD         | GitHub Actions (UV + ruff + pytest)                                                            |

## Getting Started

Each project is managed with [UV](https://docs.astral.sh/uv/). See the individual project READMEs for setup and run instructions.