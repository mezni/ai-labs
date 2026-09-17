"""Vector knowledge base implementation, backed by FAISS.

Ported from Bootcamp 3's src/knowledge/knowledge_base.py.txt and adapted to the
omni-connect local RAG stack:

    - embeddings    : local EmbeddingGenerator (BAAI/bge-small-en-v1.5, 384 dims)
    - index         : FAISS IndexFlatL2 over the embedding matrix
    - corpus glob   : data/knowledge_base/*.md.txt (from config/llm_config.yaml)

Public API is unchanged from the Bootcamp original: load_documents, _chunk_text,
build_index, save_index and load_index.
"""
import pickle
from pathlib import Path
from typing import Any, Dict, List

import faiss
import numpy as np

from src.knowledge.embeddings import EmbeddingGenerator
from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

CONFIG = load_yaml_config("config/llm_config.yaml")


class KnowledgeBase:
    """Manage vector knowledge base with FAISS."""

    def __init__(self):
        self.embedding_generator = EmbeddingGenerator()
        self.index = None
        self.documents: List[Dict[str, Any]] = []
        self.dimension = CONFIG.get("models", {}).get("embedding", {}).get("dimensions", 384)

    def load_documents(self, knowledge_base_path: str) -> List[Dict[str, Any]]:
        """Load and chunk every corpus document under `knowledge_base_path`."""
        docs = []
        kb_path = Path(knowledge_base_path)
        corpus = CONFIG.get("retrieval", {}).get("corpus", {})
        file_glob = corpus.get("glob", "*.md.txt")

        for md_file in kb_path.rglob(file_glob):
            content = md_file.read_text(encoding="utf-8")
            category = md_file.parent.name

            chunks = self._chunk_text(content, chunk_size=1000, overlap=200)

            for i, chunk in enumerate(chunks):
                docs.append({
                    "content": chunk,
                    "source": md_file.name,
                    "category": category,
                    "chunk_id": i,
                })

        return docs

    def _chunk_text(self, text: str, chunk_size: int = 1000, overlap: int = 200) -> List[str]:
        """Split text into overlapping chunks."""
        chunks = []
        start = 0
        while start < len(text):
            end = start + chunk_size
            chunks.append(text[start:end])
            start = end - overlap
        return chunks

    def build_index(self, documents: List[Dict[str, Any]]):
        """Build a FAISS index over `documents`."""
        self.documents = documents
        texts = [doc["content"] for doc in documents]

        embeddings = self.embedding_generator.generate_embeddings(texts)
        embeddings_array = np.array(embeddings).astype("float32")
        self.dimension = embeddings_array.shape[1]
        self.index = faiss.IndexFlatL2(self.dimension)
        self.index.add(embeddings_array)

        logger.info(f"Built index with {len(documents)} documents")

    def save_index(self, path: str):
        """Save the FAISS index and its documents to disk."""
        Path(path).parent.mkdir(parents=True, exist_ok=True)
        faiss.write_index(self.index, f"{path}.index")
        with open(f"{path}.docs", "wb") as f:
            pickle.dump(self.documents, f)

    def load_index(self, path: str):
        """Load a previously saved FAISS index and its documents from disk."""
        self.index = faiss.read_index(f"{path}.index")
        with open(f"{path}.docs", "rb") as f:
            self.documents = pickle.load(f)