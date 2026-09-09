# AI Experiments

A workspace of incremental AI experiments — each directory is a standalone, self-contained project. The emphasis is on understanding every component from first principles before reaching for frameworks.

## Projects

| Dir             | Status      | Description                                            |
|-----------------|-------------|--------------------------------------------------------|
| `01-rag-naive`  | Implemented | Minimal RAG indexing & retrieval pipeline (FAISS + OpenRouter + MiniMax, pure Python, `rag-indexer` CLI) |
| `02-rag-dataops`| Implemented | RAG DataOps pipeline (LlamaIndex + ChromaDB + OpenRouter, versioned indexes, lineage, rollback, `rag-dataops` CLI + Streamlit dashboard) |

Additional experiments (`00-brainstorm`, `00-notebook`, `01-rag-deploy`, `01-rag-evals`, `01-rag-guardrails`, `01-rag-monitoring`, `01-rag-advanced`) are scaffolded and planned.

## Description

The current active projects are:

**RAG DataOps Pipeline** in [`02-rag-dataops/`](02-rag-dataops/):
A DataOps-oriented Retrieval-Augmented Generation (RAG) pipeline with LlamaIndex + ChromaDB: heterogeneous document ingestion (PDF, DOCX, HTML, TXT, SQLite), deterministic SHA-256 change detection, document versioning and lineage, versioned vector snapshots with full reproducibility and physical rollback. CLI (`rag-dataops index/versions/rollback/catalog/search`) + multipage Streamlit dashboard.

**RAG Indexing & Retrieval Pipeline** in [`01-rag-naive/`](01-rag-naive/):
A lightweight, local-first Retrieval-Augmented Generation (RAG) system built in pure Python — no RAG framework. It ingests PDFs, detects new/changed/unchanged/deleted documents by SHA-256, chunks and embeds content, stores vectors in FAISS with explicit IDs and JSON metadata, and answers questions through a MiniMax generation model with source citations, all routed through OpenRouter. A `rag-indexer` CLI exposes `index`, `reindex`, `search`, and `ask`; sample policy PDFs are generated on demand by an LLM-driven script (`scripts/generate_sample_pdfs.py`).

- Design document: [`01-rag-naive/docs/PROJECT.md`](01-rag-naive/docs/PROJECT.md)
- Project README: [`01-rag-naive/README.md`](01-rag-naive/README.md)

## Tech Stack

| Layer         | Technology                                          |
|---------------|------------------------------------------------------|
| Language      | Python 3.12+                                         |
| Packaging     | UV                                                  |
| Vector store  | FAISS CPU (`faiss-cpu`), NumPy / ChromaDB (`chromadb`) |
| PDF parsing   | PyPDF                                              |
| PDF generation| reportlab (sample policy PDFs via LLM script)       |
| Validation    | Pydantic + `pydantic-settings` (config)             |
| Model access  | OpenRouter (OpenAI-compatible client, `openai`)     |
| Generation    | MiniMax (`minimax/minimax-m3`) via OpenRouter       |
| Embeddings    | OpenRouter embedding model (configurable)           |
| Storage       | JSON (metadata, document state, manifest)           |
| Config        | `python-dotenv`                                     |
| CI/CD         | GitHub Actions (UV + ruff + pytest)                 |

## Getting Started

Each project is managed with [UV](https://docs.astral.sh/uv/). See the individual project READMEs for setup and run instructions.