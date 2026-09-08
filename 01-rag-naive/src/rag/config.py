"""Centralized configuration for the RAG pipeline."""

import os
from pathlib import Path

from dotenv import load_dotenv

load_dotenv()

PROJECT_ROOT = Path(__file__).resolve().parents[2]

OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY", "")
OPENROUTER_BASE_URL = os.getenv("OPENROUTER_BASE_URL", "https://openrouter.ai/api/v1")
OPENROUTER_EMBEDDING_MODEL = os.getenv("OPENROUTER_EMBEDDING_MODEL", "")
OPENROUTER_MODEL = os.getenv("OPENROUTER_MODEL", "")

CHUNK_SIZE = 800
CHUNK_OVERLAP = 150

TOP_K = 5

DATA_DIR = PROJECT_ROOT / "data" / "raw"
INDEX_PATH = PROJECT_ROOT / "indexes" / "faiss.index"
METADATA_PATH = PROJECT_ROOT / "indexes" / "metadata.json"
DOCUMENT_STATE_PATH = PROJECT_ROOT / "indexes" / "document_state.json"
MANIFEST_PATH = PROJECT_ROOT / "indexes" / "index_manifest.json"

LOG_PATH = PROJECT_ROOT / "logs" / "indexing.log"
