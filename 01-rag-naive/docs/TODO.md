# TODO

Implementation tasks for the RAG Indexing & Retrieval Pipeline MVP.
Reference: [PROJECT.md](PROJECT.md).

## Scaffolding & Configuration

- [x] Initialize project with UV (`uv init`) and add `pypdf`, `faiss-cpu`, `numpy`, `openai`, `python-dotenv`
- [x] Create `pyproject.toml` with Python 3.12, ruff, and pytest dev dependencies
- [x] Add `.env` and `.gitignore` (`.env`, `indexes/`, `logs/`, `data/`)
- [x] Create directory skeleton: `data/raw/`, `indexes/`, `logs/`, `src/rag/`, `tests/`
- [x] Implement `src/rag/config.py` with paths, chunking, TOP_K, and OpenRouter settings
- [x] Add sample PDFs to `data/raw/` for development (`scripts/generate_sample_pdfs.py`)

## Extraction (`src/rag/extract.py`)

- [x] Implement PDF discovery over `data/raw/` (relative paths)
- [x] Implement page-aware text extraction with PyPDF
- [x] Return structured `{document_id, pages: [{page_number, text}]}` output
- [x] Handle extraction failure: raise `ExtractionError` + log (skip/preserve handled in pipeline)

## Cleaning (`src/rag/clean.py`)

- [x] Normalize line endings (`\r\n`, `\r` → `\n`)
- [x] Remove null characters (`\x00`)
- [x] Collapse excess whitespace
- [x] Preserve paragraph boundaries (blank lines collapse to a single `\n\n`)
- [x] Trim leading/trailing whitespace

## Chunking (`src/rag/chunk.py`)

- [x] Implement character-based chunking (CHUNK_SIZE=800, CHUNK_OVERLAP=150)
- [x] Track `page_start` / `page_end` per chunk
- [x] Generate deterministic `chunk_id` (`<document_id>::chunk::<index>`)
- [x] Handle empty text and short documents

## Embedding (`src/rag/embed.py`)

- [x] Create OpenAI-compatible client from `OPENROUTER_BASE_URL` / `OPENROUTER_API_KEY`
- [x] Implement embedding request against `OPENROUTER_EMBEDDING_MODEL`
- [x] Parse embedding response and extract vector
- [x] Validate embedding dimension against index dimension (mismatch → fail safely)
- [x] Add retry + logging on embedding failure

## Indexing (`src/rag/index.py` + `src/rag/state.py`)

- [x] Build `IndexIDMap2(IndexFlatIP(dim))` with explicit vector IDs
- [x] Normalize vectors with `faiss.normalize_L2` before insert and search
- [x] Implement SHA-256 hashing and change detection (NEW / UNCHANGED / CHANGED / DELETED)
- [x] Implement vector ID allocation via manifest `next_vector_id`
- [x] Add new document: chunk → embed → validate → add vectors → persist
- [x] Change document safely: validate new version first, then remove old vectors, then add new ones
- [x] Delete document: read stored vector IDs, `remove_ids`, drop metadata/state
- [x] Persist `faiss.index`, `metadata.json`, `document_state.json`, `index_manifest.json` atomically (`.tmp` + replace)
- [x] Validate index consistency (FAISS IDs ↔ metadata IDs ↔ document-state IDs) before mutations
- [x] Detect embedding-model/dimension change in manifest → trigger full reindex
- [x] Ensure idempotency: repeated runs with no changes produce zero embedding requests or vector updates

## Retrieval (`src/rag/retrieve.py`)

- [x] Embed the user question with the same embedding model, normalize it
- [x] Perform FAISS search and return top-k vector IDs with scores
- [x] Map results to metadata (vector_id → chunk_id → document_id, source, pages, text)
- [x] Return structured results including `score` (threshold/rank-ready)
- [x] Make TOP_K configurable

## Generation (`src/rag/generate.py`)

- [ ] Build system prompt: answer from context only, no invention, state when the answer is absent
- [ ] Serialize retrieved chunks into SOURCE/PAGE-aware context
- [ ] Call chat completions against `OPENROUTER_MODEL` (MiniMax)
- [ ] Return answer with source references
- [ ] Keep retrieval and generation as separate layers

## Pipeline Entrypoint (`src/rag/main.py`)

- [ ] Wire indexing stages: discover → detect → extract → clean → chunk → embed → update
- [ ] Add CLI flags: `index`, `search`, `ask`, `reindex`
- [ ] Add logging to `logs/indexing.log` (NEW/UNCHANGED/CHANGED/DELETED, counts, errors)
- [ ] Keep going after per-document failures; stop + log critical on FAISS/metadata failures

## Tests

- [x] `test_config.py`: settings defaults, env overrides, path anchoring, validation
- [x] `test_extract.py`: discovery, page-aware extraction, JSON shape, failure handling
- [x] `test_clean.py`: line endings, null bytes, whitespace, trimming, paragraph preservation
- [x] `test_chunk.py`: size, overlap, page metadata, ordering, empty text
- [x] `test_embed.py`: OpenRouter request construction, model configuration, response parsing, dimension validation, retry/failure handling
- [x] `test_state.py`: change detection NEW/UNCHANGED/CHANGED/DELETED
- [x] `test_index.py`: add/search/remove with explicit IDs, dimension validation
- [x] `test_retrieve.py`: query embedding, normalization, top-k mapping to metadata
- [ ] `test_generate.py`: prompt construction, context serialization, mock chat call
- [x] Incremental tests: no-change run → 0 embeddings; one changed doc re-embedded only; deleted doc vectors removed
- [x] Consistency test: FAISS IDs == metadata IDs; state vector IDs exist in FAISS and metadata

## CI/CD & Documentation

- [ ] Verify GitHub Actions workflow (`01-rag-naive-ci.yml`) passes (UV sync, ruff, pytest)
- [ ] Confirm `01-rag-naive` README matches final project state