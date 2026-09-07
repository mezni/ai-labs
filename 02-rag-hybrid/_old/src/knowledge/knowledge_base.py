"""Knowledge domain: document loading, chunking, and vector indexing."""
from __future__ import annotations

from pathlib import Path
from typing import Any, NotRequired, TypedDict

import chromadb
from langchain_text_splitters import RecursiveCharacterTextSplitter

from src.knowledge.embeddings import Embedder

DEFAULT_CORPUS_DIR = Path("data/policies")
DEFAULT_PERSIST_DIR = Path("data/chroma")
DEFAULT_COLLECTION = "policies"

_SENTINEL = object()


class Chunk(TypedDict):
    """A single indexed chunk with optional retrieval score."""

    chunk_id: str
    content: str
    document_id: str
    source: str
    category: str
    score: NotRequired[float]


class KnowledgeBase:
    """Knowledge base backed by a directory of markdown documents."""

    def __init__(
        self,
        corpus_dir: str | Path = DEFAULT_CORPUS_DIR,
        embedder: Embedder | None = None,
        persist_dir: str | Path = DEFAULT_PERSIST_DIR,
        collection_name: str = DEFAULT_COLLECTION,
        chunk_size: int = 1000,
        overlap: int = 100,
    ) -> None:
        self.corpus_dir = Path(corpus_dir)
        self.embedder = embedder or Embedder()
        self.persist_dir = Path(persist_dir)
        self.collection_name = collection_name
        self.chunk_size = chunk_size
        self.overlap = overlap
        self._chunks: list[dict[str, Any]] = []
        self.collection = self._open_collection(self.persist_dir)

    def _open_collection(
        self,
        persist_dir: str | Path,
        reset: bool = False,
    ) -> Any:
        client = chromadb.PersistentClient(path=str(persist_dir))
        if reset:
            for existing in client.list_collections():
                client.delete_collection(existing.name)
        return client.get_or_create_collection(
            name=self.collection_name,
            metadata={"hnsw:space": "cosine"},
        )

    def index_chunks(self, chunks: list[dict[str, Any]]) -> None:
        """Insert or update chunk records with their vectors."""
        if not chunks:
            return
        self.collection.upsert(
            ids=[chunk["chunk_id"] for chunk in chunks],
            documents=[chunk["content"] for chunk in chunks],
            embeddings=[chunk["vector"] for chunk in chunks],
            metadatas=[
                {
                    "document_id": chunk["document_id"],
                    "source": chunk["source"],
                    "category": chunk["category"],
                }
                for chunk in chunks
            ],
        )

    def build_index(self, documents: list[dict[str, Any]]) -> None:
        """Chunk and embed each document and build the vector index."""
        self._chunks = []
        for document in documents:
            document_id = document.get("document_id", document.get("source", ""))
            source = document.get("source", "")
            content = document.get("content", "")
            if not content.strip():
                continue
            records = self._chunk_text(
                content,
                document_id=document_id,
                source=source,
            )
            vectors = self.embedder.embed_documents(
                [record["content"] for record in records]
            )
            for record, vector in zip(records, vectors):
                record["vector"] = vector
            self.index_chunks(records)
            self._chunks.extend(records)

    def save_index(self, path: str) -> None:
        """Persist the built index to the given path, overwriting any existing data."""
        self.persist_dir = Path(path)
        self.collection = self._open_collection(self.persist_dir, reset=True)
        self.index_chunks(self._chunks)

    def load_documents(self) -> list[dict[str, str]]:
        """Load all markdown documents (recursively), chunk each, and return chunk records."""
        chunks: list[dict[str, str]] = []
        for path in sorted(self.corpus_dir.rglob("*.md")):
            text = path.read_text(encoding="utf-8")
            if not text.strip():
                continue
            relative_source = path.relative_to(self.corpus_dir).as_posix()
            chunks.extend(
                self._chunk_text(
                    text,
                    document_id=path.stem,
                    source=relative_source,
                )
            )
        return chunks

    def _derive_category(self, source: str) -> str:
        """Extract the top-level subdirectory of a relative source path as its category.

        `source` is expected to be relative to `corpus_dir` (e.g. "hr/leave.md").
        A bare filename with no subdirectory falls back to the corpus dir's name.
        """
        parent = Path(source).parent
        parts = parent.parts
        return parts[0] if parts else self.corpus_dir.name

    def _chunk_text(
        self,
        text: str,
        document_id: str,
        source: str,
        chunk_size: int | None = _SENTINEL,  # type: ignore[assignment]
        overlap: int | None = _SENTINEL,  # type: ignore[assignment]
    ) -> list[dict[str, str]]:
        """Split text into chunks annotated with their document metadata."""
        effective_chunk_size = (
            self.chunk_size if chunk_size is _SENTINEL else chunk_size
        )
        effective_overlap = self.overlap if overlap is _SENTINEL else overlap
        splitter = RecursiveCharacterTextSplitter(
            chunk_size=effective_chunk_size,
            chunk_overlap=effective_overlap,
        )
        category = self._derive_category(source)
        return [
            {
                "chunk_id": f"{document_id}_chunk_{i:03d}",
                "content": chunk,
                "document_id": document_id,
                "source": source,
                "category": category,
            }
            for i, chunk in enumerate(splitter.split_text(text), start=1)
        ]