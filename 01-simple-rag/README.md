# Aether Wireless - Policy RAG

A Retrieval-Augmented Generation (RAG) system that lets you ask natural language questions against Aether Wireless corporate policy documents and get grounded, sourced answers.

## Features

- **PDF Policy Generation:** Renders 15 branded A4 policy PDFs from HTML/CSS templates via WeasyPrint.
- **Document Ingestion:** Reads PDFs, chunks text with configurable overlap, and embeds via OpenRouter.
- **FAISS Vector Store:** Stores normalized embeddings for cosine-similarity search with metadata persistence.
- **Retriever:** Embeds user queries, retrieves top-K relevant chunks, and builds RAG prompts.
- **Streamlit Q&A UI:** Interactive web app for asking questions and viewing sourced answers.
- **YAML-Driven Config:** Embedding model, LLM, chunking, and vector store settings in `config/llm_config.yaml`.
- **Resilient Embeddings:** Retry with exponential backoff on 429 rate limits and connection errors.

## Project Structure

```
01-simple-rag/
├── app/
│   ├── main.py                  # Ingest pipeline (PDF → chunk → embed → FAISS)
│   └── streamlit_app.py         # Streamlit Q&A interface
├── config/
│   └── llm_config.yaml          # Embedding, LLM, chunking, vector store config
├── data/
│   ├── policies/                # Generated policy PDFs
│   └── faiss/                   # Persisted FAISS index + metadata
├── docs/
│   └── NOTES.md                 # RAG pipeline reference notes
├── scripts/
│   └── generate_docs.py         # PDF generation script
└── src/
    ├── knowledge/
    │   ├── embeddings.py        # embed() + chunk_text()
    │   ├── knowledge_base.py    # KnowledgeBase (FAISS index management)
    │   └── retriever.py         # Retriever (query → embed → search → prompt)
    └── utils/
        ├── config_loader.py     # YAML config loader
        └── logger.py            # Structured logging
```

## Tech Stack

- **Language:** Python 3.13
- **PDF Rendering:** WeasyPrint
- **Vector Store:** FAISS (cosine similarity)
- **Embeddings:** OpenRouter API (configurable model)
- **LLM:** OpenRouter API (configurable model)
- **UI:** Streamlit
- **Config:** YAML + python-dotenv
- **Tooling:** uv

## Getting Started

### Prerequisites

- Python 3.13+
- [uv](https://docs.astral.sh/uv/) package manager
- System libraries for WeasyPrint (`libpango`, `libcairo`, `gdk-pixbuf`)
- An [OpenRouter](https://openrouter.ai/) API key

### Installation

```bash
git clone https://github.com
cd 01-simple-rag
uv sync
```

### Configuration

Copy and fill in your API key:

```bash
cp .env.example .env
```

Edit `.env`:

```
OPENROUTER_API_KEY=sk-or-v1-...
```

Optional — edit `config/llm_config.yaml` to change models, chunk size, or top-K.

### Step 1: Generate Policy PDFs

```bash
uv run python scripts/generate_docs.py
```

PDFs are written to `data/policies/`.

### Step 2: Build the Vector Store

```bash
python app/main.py
```

Reads all PDFs, chunks, embeds, and saves the FAISS index to `data/faiss/`.

### Step 3: Launch the Q&A App

```bash
streamlit run app/streamlit_app.py
```

Opens a browser UI where you can ask questions about the policies.

## Configuration Reference

All settings live in `config/llm_config.yaml`:

| Section | Key | Description |
|---------|-----|-------------|
| `embeddings` | `model` | Embedding model ID |
| `embeddings` | `batch_size` | Embedding batch size |
| `chunking` | `chunk_size` | Characters per chunk |
| `chunking` | `overlap` | Overlap between chunks |
| `vector_store` | `dir` | FAISS index directory |
| `llm` | `model` | Chat/completion model ID |
| `llm` | `temperature` | Generation temperature |
| `llm` | `max_tokens` | Max output tokens |

## License

Distributed under the MIT License. See `LICENSE` for more information.
