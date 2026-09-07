"""Knowledge base for the vector RAG pipeline.

Manages document loading, chunking, embedding, and the FAISS vector index.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any, NotRequired, TypedDict

import faiss
import numpy as np

from src.knowledge.embeddings import EmbeddingGenerator
from src.utils import get_logger

logger = get_logger(__name__)

DEFAULT_CORPUS_DIR = Path("data/policies")
DEFAULT_INDEX_PATH = Path("data/faiss/index.bin")
DEFAULT_CHUNKS_PATH = Path("data/faiss/chunks.json")


class Chunk(TypedDict):
    """A single indexed chunk with optional retrieval score."""

    chunk_id: str
    content: str
    document_id: str
    source: str
    category: str
    vector: NotRequired[list[float]]
    score: NotRequired[float]


class KnowledgeBase:
    """Vector knowledge base backed by a FAISS index over embedded chunks."""

    def __init__(
        self,
        corpus_dir: str | Path = DEFAULT_CORPUS_DIR,
        embedder: EmbeddingGenerator | None = None,
        index_path: str | Path = DEFAULT_INDEX_PATH,
        chunks_path: str | Path = DEFAULT_CHUNKS_PATH,
        chunk_size: int = 1000,
        overlap: int = 100,
    ) -> None:
        self.corpus_dir = Path(corpus_dir)
        self.embedder = embedder or EmbeddingGenerator()
        self.index_path = Path(index_path)
        self.chunks_path = Path(chunks_path)
        self.chunk_size = chunk_size
        self.overlap = overlap
        self.chunks: list[Chunk] = []
        self._index: faiss.Index | None = None

    # -- index lifecycle ------------------------------------------------------
    def _create_index(self, dimension: int) -> faiss.Index:
        """Cosine similarity via inner product over L2-normalized vectors."""
        self._index = faiss.IndexFlatIP(dimension)
        return self._index

    def _ensure_index(self, dimension: int) -> faiss.Index:
        if self._index is None:
            self._create_index(dimension)
        return self._index

    def is_built(self) -> bool:
        return self._index is not None and self._index.ntotal > 0

    def __len__(self) -> int:
        return int(self._index.ntotal) if self._index is not None else 0

    def reset_index(self) -> None:
        """Drop the in-memory index and all stored chunk metadata."""
        self._index = None
        self.chunks = []
        logger.info("Vector index reset")

    # -- building -------------------------------------------------------------
    def build_index(self, documents: list[dict[str, Any]]) -> None:
        """Chunk and embed each document and build the FAISS index."""
        self.reset_index()
        pending: list[Chunk] = []
        for document in documents:
            document_id = document.get("document_id", document.get("source", ""))
            source = document.get("source", "")
            content = document.get("content", "")
            if not content.strip():
                logger.debug("Skipping empty document %r", document_id)
                continue
            pending.extend(
                self._chunk_text(content, document_id=document_id, source=source)
            )
        self.index_chunks(pending)

    def index_chunks(self, chunks: list[Chunk]) -> None:
        """Embed any chunks lacking vectors and add them to the FAISS index."""
        if not chunks:
            return

        pending = [c for c in chunks if not c.get("vector")]
        if pending:
            vectors = self.embedder.embed_documents(
                [c["content"] for c in pending]
            )
            for chunk, vector in zip(pending, vectors):
                chunk["vector"] = vector

        if self._index is None:
            self._create_index(len(chunks[0]["vector"]))
        matrix = self._normalize_points([c["vector"] for c in chunks])
        self._index.add(np.asarray(matrix, dtype=np.float32))
        self.chunks.extend(
            {k: v for k, v in chunk.items() if k != "vector"} for chunk in chunks
        )
        logger.info("Indexed %d chunks (%d total)", len(chunks), len(self))

    # -- querying -------------------------------------------------------------
    def search(self, query: str, top_k: int = 5) -> list[Chunk]:
        """Embed the query and return the top_k nearest chunks with scores."""
        if not self.is_built():
            raise RuntimeError(
                "Knowledge base is empty; call build_index() before searching"
            )
        query_vector = self.embedder.embed_text(query)
        query_matrix = np.asarray(
            [self._normalize_point(query_vector)], dtype=np.float32
        )
        scores, indices = self._index.search(query_matrix, top_k)

        results: list[Chunk] = []
        for score, idx in zip(scores[0], indices[0]):
            if idx < 0 or int(idx) >= len(self.chunks):
                continue
            chunk = dict(self.chunks[int(idx)])
            chunk["score"] = float(score)
            results.append(chunk)
        logger.debug("Search %r returned %d chunks", query, len(results))
        return results

    # -- persistence ----------------------------------------------------------
    def save_index(self, index_path: str | Path | None = None, chunks_path: str | Path | None = None) -> None:
        """Persist the FAISS index and chunk metadata to disk."""
        if not self.is_built():
            raise RuntimeError("Nothing to save; build the index first")
        index_path = Path(index_path) if index_path else self.index_path
        chunks_path = Path(chunks_path) if chunks_path else self.chunks_path
        index_path.parent.mkdir(parents=True, exist_ok=True)
        chunks_path.parent.mkdir(parents=True, exist_ok=True)

        faiss.write_index(self._index, str(index_path))
        chunks_path.write_text(
            json.dumps(self.chunks, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
        logger.info("Saved index (%d vectors) to %s", len(self), index_path)

    def load_index(self, index_path: str | Path | None = None, chunks_path: str | Path | None = None) -> None:
        """Load a FAISS index and chunk metadata from disk."""
        index_path = Path(index_path) if index_path else self.index_path
        chunks_path = Path(chunks_path) if chunks_path else self.chunks_path
        if not index_path.exists() or not chunks_path.exists():
            raise FileNotFoundError(
                f"Index files not found at {index_path} and {chunks_path}"
            )

        self._index = faiss.read_index(str(index_path))
        self.chunks = json.loads(chunks_path.read_text(encoding="utf-8"))
        logger.info("Loaded index (%d vectors) from %s", len(self), index_path)

    # -- corpus loading -------------------------------------------------------
    def load_documents(self) -> list[dict[str, str]]:
        """Load all markdown documents (recursively) as raw documents."""
        documents: list[dict[str, str]] = []
        for path in sorted(self.corpus_dir.rglob("*.md")):
            text = path.read_text(encoding="utf-8")
            if not text.strip():
                logger.debug("Skipping empty file %s", path)
                continue
            documents.append(
                {
                    "document_id": path.stem,
                    "source": path.relative_to(self.corpus_dir).as_posix(),
                    "content": text,
                }
            )
        logger.info("Loaded %d documents from %s", len(documents), self.corpus_dir)
        return documents

    def _derive_category(self, source: str) -> str:
        """Extract the top-level subdirectory of a relative source path."""
        parts = Path(source).parent.parts
        return parts[0] if parts else self.corpus_dir.name

    def _chunk_text(
        self,
        text: str,
        document_id: str,
        source: str,
        chunk_size: int | None = None,
        overlap: int | None = None,
    ) -> list[Chunk]:
        """Split text into overlapping character-based chunks."""
        size = self.chunk_size if chunk_size is None else chunk_size
        step = max(size - (self.overlap if overlap is None else overlap), 1)
        category = self._derive_category(source)
        chunks: list[Chunk] = []
        for i, start in enumerate(range(0, len(text), step), start=1):
            chunk = text[start : start + size]
            if not chunk.strip():
                continue
            chunks.append(
                {
                    "chunk_id": f"{document_id}_chunk_{i:03d}",
                    "content": chunk,
                    "document_id": document_id,
                    "source": source,
                    "category": category,
                }
            )
        return chunks

    # -- helpers --------------------------------------------------------------
    @staticmethod
    def _normalize_point(vector: list[float] | np.ndarray) -> np.ndarray:
        arr = np.asarray(vector, dtype=np.float32)
        norm = np.linalg.norm(arr)
        return arr if norm == 0 else arr / norm

    def _normalize_points(self, vectors: list[list[float]]) -> np.ndarray:
        return np.vstack([self._normalize_point(v) for v in vectors])

    def close(self) -> None:
        if isinstance(self.embedder, EmbeddingGenerator):
            self.embedder.close()

    def __enter__(self) -> "KnowledgeBase":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()