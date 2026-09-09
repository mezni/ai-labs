# TODO

Implementation tasks for the RAG DataOps pipeline (LlamaIndex + ChromaDB).
Reference: [PROJECT.md](PROJECT.md).

## Phase 0 — Scaffolding & Configuration

- [ ] Initialize project with UV and add `llama-index-core`, `llama-index-vector-stores-chroma`, `chromadb`, `pydantic`, `pydantic-settings`, `openai`, `pypdf`, `python-docx`, `beautifulsoup4`, `python-dotenv`
- [ ] Add `streamlit` as a `dashboard` extra in `pyproject.toml` (not a core runtime dependency)
- [ ] Create `pyproject.toml` with Python 3.12, ruff, and pytest dev dependencies
- [ ] Add `.env.example` and `.gitignore` (`.env`, `data/raw/`, `data/processed/`, `indexes/`, `logs/`)
- [ ] Create directory skeleton: `data/raw/`, `indexes/chroma/`, `logs/`, `src/{config,main}.py`, `src/ingestion/`, `src/chunking/`, `src/embeddings/`, `src/indexing/`, `src/retrieval/`, `tests/`
- [ ] Implement `src/config.py` with OpenRouter, Chroma, registry/catalog paths, chunking, and TOP_K settings
- [ ] Write ADR-001: LlamaIndex + ChromaDB as the RAG stack (vs. pure-Python FAISS)
- [ ] Write ADR-002: full snapshots per index version (physical rollback), append-only registry
- [ ] Add sample documents to `data/raw/` for development (`scripts/generate_sample_docs.py`)

## RAGOps 1 — Document Hashing (change detection + idempotency)

### Hashing (`src/ingestion/hashing.py`)

- [ ] Implement streaming SHA-256 hashing of document content (file bytes / parsed text)
- [ ] Implement `Change` enum: NEW / UNCHANGED / CHANGED / DELETED
- [ ] Implement change detection from current hashes vs. previous catalog
- [ ] Enforce idempotency: repeated runs with no document changes produce no new embeddings or snapshots

### Parsing (`src/ingestion/parser.py`)

- [ ] Implement PDF parser with `pypdf`
- [ ] Implement DOCX parser with `python-docx`
- [ ] Implement HTML parser with `beautifulsoup4`
- [ ] Implement TXT parser
- [ ] Implement database-row parser (rows projected to text documents; MVP: SQLite)
- [ ] Return normalized `{document_id, source, format, text, metadata, collected_at}` output
- [ ] Handle parse failure: raise + log (skip handled in pipeline)

### Loading (`src/ingestion/loader.py`)

- [ ] Implement source profiles (filesystem, web, confluence, sharepoint, database)
- [ ] Implement recursive filesystem discovery (PDF/DOCX/TXT)
- [ ] Route each discovered document to the matching parser
- [ ] Pass through source metadata for lineage capture
- [ ] Register connectors as a source registry (filesystem + database wired; confluence/sharepoint/web connector profiles)

### Chunking (`src/chunking/chunker.py`)

- [ ] Wrap LlamaIndex `NodeParser` with `CHUNK_SIZE=800` / `CHUNK_OVERLAP=150`
- [ ] Generate deterministic `chunk_id` (`{document_id}::{version}::chunk::{index}`)
- [ ] Track `block_start` / `block_end` offsets per chunk
- [ ] Handle empty text and short documents

### Embeddings (`src/embeddings/embedder.py`)

- [ ] Create OpenAI-compatible client from `OPENROUTER_BASE_URL` / `OPENROUTER_API_KEY`
- [ ] Register embedding model id and capture embedding dimension (recorded on first embedding)
- [ ] Validate dimension consistency (mismatch → log critical, do not add vector, do not persist snapshot)
- [ ] Detect embedding-model change → existing snapshot incompatible → full reindex
- [ ] Add retry + logging on embedding failure

### Index Build — minimal (`src/indexing/builder.py`)

- [ ] Build a snapshot into a versioned Chroma collection via `ChromaVectorStore`
- [ ] Persist chunk nodes with provenance metadata
- [ ] Return snapshot stats (chunk count, document count, per-source counts)

### CLI — index (`src/main.py`)

- [ ] Implement `rag-dataops index`: discover → hash → change-detect → parse → chunk → embed → build snapshot → bump version
- [ ] Skip UNCHANGED documents with no embedding requests
- [ ] Replace CHANGED documents in the snapshot
- [ ] Remove DELETED documents from the snapshot
- [ ] Keep going after per-document failures; stop + log critical on snapshot failures (registry not mutated)

## RAGOps 2 — Document Metadata (lineage + provenance)

### Document Catalog (`src/ingestion/` + registry storage)

- [ ] Implement `document_catalog.json` persistence (version, hash, last_modified, source, format, chunk_ids)
- [ ] Track document versions derived from hash history (e.g. `v4`)
- [ ] Record version history so lineage can be traced across index snapshots

### Metadata Extraction & Lineage

- [ ] Capture source metadata at ingestion time (author, url, table, row keys, collected_at, ...)
- [ ] Stamp every chunk node with provenance fields: document_id, version, source, format, chunk_index, block_start, block_end, index_version, chunk_hash
- [ ] Compute deterministic `chunk_hash` for cross-snapshot chunk recognition
- [ ] Implement lineage lookup: chunk → document → version → source → index snapshot → embedding model/dimension

### CLI — catalog (`src/main.py`)

- [ ] Implement `rag-dataops catalog` showing document versions, hashes, and lineage

## RAGOps 3 — Index Versioning (reproducibility + rollback)

### Index Registry (`src/indexing/registry.py`)

- [ ] Implement `index_registry.json` persistence: `current_version` pointer + append-only version records
- [ ] Record per version: created_at, embedding_model, embedding_dimension, embedding_provider, chunk_size, chunk_overlap, collection_name, chunk_count, document_count, per-source stats
- [ ] Enforce immutability of existing version records (only `current_version` may change)

### Versioning (`src/indexing/versioning.py`)

- [ ] Implement snapshot lifecycle: build snapshot → create versioned Chroma collection → write version config → point `current_version`
- [ ] Ensure a failed snapshot build does not mutate the registry or `current_version`
- [ ] Implement rollback: verify target snapshot exists, repoint `current_version`, validate restored collection
- [ ] Fail loudly if the target snapshot is missing on disk

### Retrieval (`src/retrieval/retriever.py`)

- [ ] Embed the query with the registered embedding model
- [ ] Search the current versioned Chroma collection via LlamaIndex
- [ ] Require query embedding model to match the registered snapshot model
- [ ] Return top-k chunks with lineage metadata

### CLI — full (`src/main.py`)

- [ ] Implement `rag-dataops versions` listing index versions + configuration
- [ ] Implement `rag-dataops rollback <version>`
- [ ] Implement `rag-dataops search "<query>"` returning chunks with lineage metadata

> **Out of scope for this version:** retrieval metrics (Recall@k, MRR, Precision@k) — deferred to the Roadmap (`docs/PROJECT.md` §21).

## Streamlit Dashboard (`app/`)

- [ ] Scaffold the multipage app: `app/app.py` entrypoint + `app/pages/` (Documents, Index Versions, Lineage, Search, Rollback)
- [ ] Overview page (`app.py`): current version, chunk/document counts, per-source distribution
- [ ] Documents page: catalog entries (version, hash, last_modified, source, format, chunk_ids) rendered as a table with filtering
- [ ] Index Versions page: registry versions + configuration (embedding model, dimension, provider, chunk size/overlap, counts)
- [ ] Lineage page: drill from chunk → document → version → source connector → index snapshot
- [ ] Search page: question input, top-k results with lineage metadata (calls `src/retrieval/retriever.py`)
- [ ] Rollback page: version selector, confirmation, calls `src/indexing/versioning.py` rollback (enforces snapshot-exists guardrail)
- [ ] Read `indexes/index_registry.json` and `indexes/document_catalog.json` for all read-only pages
- [ ] Keep dashboard read-only except the Rollback page; no indexing from the UI

## Logging

- [ ] Log to `logs/rag-dataops.log` and console (INFO): discovery, NEW/CHANGED, snapshot build, version registration, rollback
- [ ] Log errors: parse failure, embedding failure, dimension mismatch, snapshot build failure, rollback failure

## Tests

- [ ] `test_config.py`: settings defaults, env overrides, path anchoring, validation
- [ ] `test_hashing.py`: NEW/UNCHANGED/CHANGED/DELETED detection; idempotency of repeat runs; streaming hash determinism
- [ ] `test_parser.py`: PDF, DOCX, HTML, TXT, database-row parsing into normalized documents; failure handling
- [ ] `test_loader.py`: recursive discovery, connector routing, source metadata passthrough
- [ ] `test_chunker.py`: chunk size, overlap, provenance metadata, chunk identity, empty text
- [ ] `test_embedder.py`: model registration, dimension capture, dimension validation, model-change detection
- [ ] `test_builder.py`: snapshot build into versioned collection, chunk/document counts, per-source stats
- [ ] `test_registry.py`: version catalog read/write, immutability of existing version records, `current_version` pointer
- [ ] `test_versioning.py`: snapshot isolation, rollback restores correct collection, failed rollout leaves state intact, missing snapshot fails loudly
- [ ] `test_retriever.py`: query embedding, model-match enforcement, top-k results with lineage metadata
- [ ] `test_main.py`: CLI dispatch (index, versions, rollback, catalog, search); per-document failure continues vs. critical aborts; unchanged-run idempotency; logging setup
- [ ] `test_dashboard.py`: pages import and render against sample catalog/registry JSON (smoke test)

## CI/CD & Documentation

- [ ] Verify GitHub Actions workflow passes (UV sync, ruff, pytest)
- [ ] `test_readme.py`: README structure references, CLI subcommands, and CI gate stay in sync with the package
- [ ] Refresh README / PROJECT / TODO to match the implemented state at each phase boundary
- [ ] Add `docs/PROJECT.md` markdown linting/consistency check to CI (optional)