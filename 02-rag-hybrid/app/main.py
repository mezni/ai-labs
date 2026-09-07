"""Knowledge base for the RAG hybrid pipeline."""
from __future__ import annotations

import os
import re
from abc import ABC, abstractmethod
from pathlib import Path
from typing import Any, NotRequired, TypedDict

import chromadb
import httpx
from dotenv import load_dotenv
from langchain_text_splitters import RecursiveCharacterTextSplitter
from rank_bm25 import BM25Okapi

from src.utils.config_loader import load_all_configs

load_dotenv()

DEFAULT_CORPUS_DIR = Path("data/policies")
DEFAULT_PERSIST_DIR = Path("data/chroma")
DEFAULT_COLLECTION = "policies"


class KnowledgeBase:
    """Knowledge base backed by a directory of markdown documents."""

    def __init__(
        self,
        corpus_dir: str | Path = DEFAULT_CORPUS_DIR,
        embedder: BaseEmbedder | None = None,
        persist_dir: str | Path = DEFAULT_PERSIST_DIR,
        collection_name: str = DEFAULT_COLLECTION,
        chunk_size: int = 1000,
        overlap: int = 100,
    ) -> None:
        self.corpus_dir = Path(corpus_dir)
        self.embedder = embedder or LocalEmbedder()
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
        """Load all markdown documents, check and chunk each, and return the chunk records."""
        chunks: list[dict[str, str]] = []
        for path in sorted(self.corpus_dir.glob("*.md")):
            text = path.read_text(encoding="utf-8")
            if not text.strip():
                continue
            chunks.extend(
                self._chunk_text(
                    text,
                    document_id=path.stem,
                    source=path.name,
                )
            )
        return chunks

    def _derive_category(self, source: str) -> str:
        """Extract the parent directory name of a file as its category tag."""
        parent = Path(source).parent
        return parent.name if parent.name else self.corpus_dir.name

    def _chunk_text(
        self,
        text: str,
        document_id: str,
        source: str,
        chunk_size: int | None = None,
        overlap: int | None = None,
    ) -> list[dict[str, str]]:
        """Split text into chunks annotated with their document metadata."""
        splitter = RecursiveCharacterTextSplitter(
            chunk_size=chunk_size or self.chunk_size,
            chunk_overlap=overlap or self.overlap,
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


class BaseEmbedder(ABC):
    """Common interface for all embedding providers."""

    def embed_text(self, text: str) -> list[float]:
        """Embed a single text into a vector."""
        return self.embed_documents([text])[0]

    @abstractmethod
    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed a list of texts into vectors."""


class OpenRouterEmbedder(BaseEmbedder):
    """Embed text into vectors using the configured API embeddings provider."""

    def __init__(self) -> None:
        configs = load_all_configs()
        self._cfg = configs.get("llm", {}).get("embeddings", {})
        self.model = self._cfg["model"]
        self.url = self._cfg.get(
            "openrouter_url", "https://openrouter.ai/api/v1/embeddings"
        )
        self.batch_size = self._cfg.get("batch_size", 64)
        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")
        self._client = httpx.Client(
            timeout=self._cfg.get("timeout_seconds", 120),
            headers={"Authorization": f"Bearer {api_key}"},
        )

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed a list of texts into vectors, batched per the config."""
        if not texts:
            return []
        vectors: list[list[float]] = []
        for i in range(0, len(texts), self.batch_size):
            batch = texts[i : i + self.batch_size]
            response = self._client.post(
                self.url,
                json={"model": self.model, "input": batch},
            )
            response.raise_for_status()
            data = response.json()["data"]
            vectors.extend(item["embedding"] for item in data)
        return vectors


class LocalEmbedder(BaseEmbedder):
    """Embed text into vectors locally with sentence-transformers."""

    def __init__(self, model_name: str = "all-MiniLM-L6-v2") -> None:
        from sentence_transformers import SentenceTransformer

        self._model = SentenceTransformer(model_name)

    def embed_documents(self, texts: list[str]) -> list[list[float]]:
        """Embed a list of texts into vectors."""
        if not texts:
            return []
        return [vec.tolist() for vec in self._model.encode(texts)]


class Chunk(TypedDict):
    """A single indexed chunk with optional retrieval score."""

    chunk_id: str
    content: str
    document_id: str
    source: str
    category: str
    score: NotRequired[float]


class Retriever:
    """Hybrid retriever fusing BM25 (sparse) and vector (dense) search via RRF."""

    def __init__(
        self,
        collection: Any,
        embedder: BaseEmbedder,
        chunks: list[Chunk],
        sparse_top_k: int = 15,
        dense_top_k: int = 15,
    ) -> None:
        self.collection = collection
        self.embedder = embedder
        self.sparse_top_k = sparse_top_k
        self.dense_top_k = dense_top_k
        if not chunks:
            raise ValueError("chunks must not be empty")
        self._chunk_ids = [chunk["chunk_id"] for chunk in chunks]
        self._chunks_by_id = {chunk["chunk_id"]: chunk for chunk in chunks}
        self._bm25 = self._build_bm25(chunks)

    @staticmethod
    def _tokenize(text: str) -> list[str]:
        return re.findall(r"[a-z0-9]+", text.lower())

    def _build_bm25(self, chunks: list[Chunk]) -> BM25Okapi:
        return BM25Okapi([self._tokenize(chunk["content"]) for chunk in chunks])

    def _dense_query(self, text: str, top_k: int) -> list[dict[str, object]]:
        """Semantic search over the vector collection."""
        vector = self.embedder.embed_text(text)
        result = self.collection.query(
            query_embeddings=[vector],
            n_results=top_k,
            include=["documents", "metadatas", "distances"],
        )
        hits: list[dict[str, object]] = []
        for chunk_id, doc, metadata, distance in zip(
            result["ids"][0],
            result["documents"][0],
            result["metadatas"][0],
            result["distances"][0],
        ):
            hits.append(
                {
                    "chunk_id": chunk_id,
                    "content": doc,
                    "document_id": metadata["document_id"],
                    "source": metadata["source"],
                    "category": metadata["category"],
                    "score": 1.0 - distance,
                }
            )
        return hits

    def retrieve(self, query: str, top_k: int = 5) -> list[Chunk]:
        """Return the top-k chunks for the query, ranked by RRF fusion."""
        tokens = self._tokenize(query)
        sparse_scores = self._bm25.get_scores(tokens)
        sparse_ranks = sorted(
            range(len(self._chunk_ids)),
            key=lambda i: sparse_scores[i],
            reverse=True,
        )[: self.sparse_top_k]
        dense_hits = self._dense_query(query, self.dense_top_k)
        dense_hits_by_id = {hit["chunk_id"]: hit for hit in dense_hits}

        rrf_scores: dict[str, float] = {}
        for rank, hit in enumerate(dense_hits, start=1):
            chunk_id = hit["chunk_id"]
            rrf_scores[chunk_id] = rrf_scores.get(chunk_id, 0.0) + 1.0 / (60 + rank)
        for rank, idx in enumerate(sparse_ranks, start=1):
            chunk_id = self._chunk_ids[idx]
            rrf_scores[chunk_id] = rrf_scores.get(chunk_id, 0.0) + 1.0 / (60 + rank)

        results: list[Chunk] = []
        for chunk_id, score in sorted(
            rrf_scores.items(), key=lambda item: item[1], reverse=True
        )[:top_k]:
            chunk = self._chunks_by_id.get(chunk_id) or dict(dense_hits_by_id[chunk_id])
            copy = dict(chunk)
            copy["score"] = score
            results.append(copy)
        return results


SYSTEM_PROMPT = (
    "You answer questions using only the provided context. "
    "If the answer cannot be found in the context, "
    "say that you don't have enough information."
)


def build_generation_prompt(
    query: str,
    context: list[Chunk],
) -> list[dict[str, str]]:
    """Build a chat prompt combining retrieved context chunks with the query."""
    context_text = "\n\n".join(
        f"[Chunk {i}]\n{chunk['content']}"
        for i, chunk in enumerate(context, start=1)
    )
    user_prompt = f"CONTEXT:\n{context_text}\n\nQUESTION:\n{query}"
    return [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": user_prompt},
    ]


OPENROUTER_CHAT_URL = "https://openrouter.ai/api/v1/chat/completions"


class Generator:
    """Generate grounded answers by calling the configured LLM."""

    def __init__(self) -> None:
        configs = load_all_configs()
        models = configs.get("llm", {}).get("models") or [{}]
        model_cfg = models[0]
        self.model = model_cfg.get("model", "gpt-4o-mini")
        self.url = model_cfg.get("openrouter_url", OPENROUTER_CHAT_URL)
        self.max_tokens = model_cfg.get("max_tokens", 4096)
        self.temperature = model_cfg.get("temperature", 0.3)
        api_key = os.getenv("OPENROUTER_API_KEY")
        if not api_key:
            raise RuntimeError("OPENROUTER_API_KEY is not set")
        self._client = httpx.Client(
            timeout=model_cfg.get("timeout_seconds", 120),
            headers={"Authorization": f"Bearer {api_key}"},
        )

    def generate(self, messages: list[dict[str, str]]) -> str:
        """Send the chat messages to the LLM and return the reply."""
        response = self._client.post(
            self.url,
            json={
                "model": self.model,
                "max_tokens": self.max_tokens,
                "temperature": self.temperature,
                "messages": messages,
            },
        )
        response.raise_for_status()
        return response.json()["choices"][0]["message"]["content"]


def answer(
    query: str,
    retriever: Retriever,
    generator: Generator,
    top_k: int = 5,
) -> str:
    """Run the full RAG pipeline: retrieve, build a prompt, and generate an answer."""
    context = retriever.retrieve(query, top_k=top_k)
    prompt = build_generation_prompt(query, context)
    return generator.generate(prompt)