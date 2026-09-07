"""Knowledge base for the RAG hybrid pipeline.

Manages document loading, chunking, and the ChromaDB vector index.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any, TypedDict, NotRequired

import chromadb
from langchain_text_splitters import RecursiveCharacterTextSplitter

from src.knowledge.embeddings import EmbeddingGenerator
from src.utils.logger import get_logger

logger = get_logger(__name__)

DEFAULT_CORPUS_DIR = Path("data/policies")
DEFAULT_PERSIST_DIR = Path("data/chroma")
DEFAULT_COLLECTION = "policies"


class Chunk(TypedDict):
    """A single indexed chunk with optional retrieval score and vector."""

    chunk_id: str
    content: str
    document_id: str
    source: str
    category: str
    vector: NotRequired[list[float]]
    score: NotRequired[float]


class KnowledgeBase:
    """Knowledge base backed by a directory of markdown documents."""

    def __init__(
        self,
        corpus_dir: str | Path = DEFAULT_CORPUS_DIR,
        embedder: EmbeddingGenerator | None = None,
        persist_dir: str | Path = DEFAULT_PERSIST_DIR,
        collection_name: str = DEFAULT_COLLECTION,
        chunk_size: int = 1000,
        overlap: int = 100,
    ) -> None:
        self.corpus_dir = Path(corpus_dir)
        self.embedder = embedder or EmbeddingGenerator()
        self.persist_dir = Path(persist_dir)
        self.collection_name = collection_name
        self.chunk_size = chunk_size
        self.overlap = overlap
        self.chunks: list[Chunk] = []
        self._client: Any = None
        self._client_persist_dir: Path | None = None
        self.collection = self._open_collection(self.persist_dir)

    def _open_collection(
        self,
        persist_dir: str | Path,
        reset: bool = False,
    ) -> Any:
        persist = Path(persist_dir)
        if self._client is None or persist != self._client_persist_dir:
            self._client = chromadb.PersistentClient(path=str(persist))
            self._client_persist_dir = persist
        if reset:
            for existing in self._client.list_collections():
                logger.info("Deleting existing collection %r", existing.name)
                self._client.delete_collection(existing.name)
        collection = self._client.get_or_create_collection(
            name=self.collection_name,
            metadata={"hnsw:space": "cosine"},
        )
        logger.debug("Opened collection %r at %s", self.collection_name, persist)
        return collection

    def reset_collection(self) -> None:
        """Delete and recreate the vector collection for a fresh rebuild."""
        self.collection = self._open_collection(self.persist_dir, reset=True)

    def index_chunks(self, chunks: list[Chunk]) -> None:
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
        logger.info("Indexed %d chunks into %r", len(chunks), self.collection_name)

    def build_index(self, documents: list[dict[str, Any]]) -> None:
        """Chunk and embed each document and build the vector index."""
        self.chunks = []
        for document in documents:
            document_id = document.get("document_id", document.get("source", ""))
            source = document.get("source", "")
            content = document.get("content", "")
            if not content.strip():
                logger.debug("Skipping empty document %r", document_id)
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
            self.chunks.extend(records)

    def save_index(self, path: str) -> None:
        """Persist the built index to the given path, overwriting any existing data."""
        self.persist_dir = Path(path)
        self.reset_collection()
        self.index_chunks(self.chunks)

    def load_documents(self) -> list[dict[str, str]]:
        """Load all markdown documents (recursively) as raw documents.

        Each entry carries `document_id`, `source`, and `content`; chunking
        and embedding happen later in `build_index`.
        """
        documents: list[dict[str, str]] = []
        for path in sorted(self.corpus_dir.rglob("*.md")):
            text = path.read_text(encoding="utf-8")
            if not text.strip():
                logger.debug("Skipping empty file %s", path)
                continue
            relative_source = path.relative_to(self.corpus_dir).as_posix()
            documents.append(
                {
                    "document_id": path.stem,
                    "source": relative_source,
                    "content": text,
                }
            )
        logger.info("Loaded %d documents from %s", len(documents), self.corpus_dir)
        return documents

    def _derive_category(self, source: str) -> str:
        """Extract the top-level subdirectory of a relative source path as its category."""
        parent = Path(source).parent
        parts = parent.parts
        return parts[0] if parts else self.corpus_dir.name

    def _chunk_text(
        self,
        text: str,
        document_id: str,
        source: str,
        chunk_size: int | None = None,
        overlap: int | None = None,
    ) -> list[Chunk]:
        """Split text into chunks annotated with their document metadata."""
        effective_chunk_size = self.chunk_size if chunk_size is None else chunk_size
        effective_overlap = self.overlap if overlap is None else overlap
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