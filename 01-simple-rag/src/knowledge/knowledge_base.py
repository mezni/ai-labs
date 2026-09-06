import json
from pathlib import Path

import faiss
import numpy as np

from src.knowledge.embeddings import chunk_text
from src.utils.config_loader import load_yaml_config
from src.utils.logger import get_logger

log = get_logger(__name__)

_config = load_yaml_config("config/llm_config.yaml")
_emb_cfg = _config["embeddings"]
_vs_cfg = _config["vector_store"]


class KnowledgeBase:
    def __init__(
        self,
        index_path: Path | str | None = None,
        metadata_path: Path | str | None = None,
    ) -> None:
        self._index_path = Path(
            index_path or f"{_vs_cfg['dir']}/{_vs_cfg['index_file']}"
        )
        self._metadata_path = Path(
            metadata_path or f"{_vs_cfg['dir']}/{_vs_cfg['metadata_file']}"
        )

        self.index: faiss.Index | None = None
        self.metadata: list[dict[str, str]] = []

        if self._index_path.exists():
            self.load()

    def load(self) -> None:
        self.index = faiss.read_index(str(self._index_path))
        with open(self._metadata_path, encoding="utf-8") as f:
            self.metadata = json.load(f)
        log.info(
            "Loaded FAISS index (%d vectors) from %s",
            self.index.ntotal,
            self._index_path,
        )

    def save(self) -> None:
        if self.index is None:
            raise RuntimeError("No index to save — call build() first")
        self._index_path.parent.mkdir(parents=True, exist_ok=True)
        faiss.write_index(self.index, str(self._index_path))
        with open(self._metadata_path, "w", encoding="utf-8") as f:
            json.dump(self.metadata, f, ensure_ascii=False, indent=2)
        log.info(
            "Saved FAISS index (%d vectors) to %s",
            self.index.ntotal,
            self._index_path,
        )

    def build(
        self,
        documents: list[dict[str, str]],
        embed_fn,
        embedding_batch_size: int = _emb_cfg["batch_size"],
    ) -> None:
        if not documents:
            raise ValueError("documents list is empty — nothing to embed")

        chunks_with_meta: list[dict[str, str]] = []
        for doc in documents:
            for chunk in chunk_text(doc["text"]):
                chunks_with_meta.append(
                    {
                        "filename": doc["filename"],
                        "text": chunk,
                    }
                )

        flat_vectors: list[np.ndarray] = []
        for start in range(0, len(chunks_with_meta), embedding_batch_size):
            batch_texts = [
                item["text"]
                for item in chunks_with_meta[start : start + embedding_batch_size]
            ]
            vectors = embed_fn(batch_texts)
            for vector in vectors:
                flat_vectors.append(np.array(vector, dtype=np.float32))

        embedding_dim = len(flat_vectors[0])
        index = faiss.IndexFlatIP(embedding_dim)
        matrix = np.vstack(flat_vectors)
        faiss.normalize_L2(matrix)
        index.add(matrix)

        self.index = index
        self.metadata = chunks_with_meta
        self.save()

    def search(
        self,
        query_embedding: list[float],
        top_k: int = 5,
    ) -> list[dict[str, str]]:
        if self.index is None:
            raise RuntimeError("No index loaded — call load() or build() first")

        query = np.array([query_embedding], dtype=np.float32)
        faiss.normalize_L2(query)
        scores, indices = self.index.search(query, top_k)

        results = []
        for score, idx in zip(scores[0], indices[0]):
            if idx == -1:
                continue
            results.append(
                {
                    "score": float(score),
                    "filename": self.metadata[idx]["filename"],
                    "text": self.metadata[idx]["text"],
                }
            )
        return results