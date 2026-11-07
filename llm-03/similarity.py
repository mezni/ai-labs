import numpy as np
from sentence_transformers import SentenceTransformer


class EmbeddingEngine:
    def __init__(
        self,
        model_name: str = "sentence-transformers/all-MiniLM-L6-v2",
    ):
        self.model = SentenceTransformer(model_name)

    def embed(self, texts: list[str]) -> np.ndarray:
        if not texts:
            raise ValueError("Texts cannot be empty")

        if any(not text.strip() for text in texts):
            raise ValueError("Texts cannot contain empty values")

        return self.model.encode(
            texts,
            normalize_embeddings=True,
        )

    @staticmethod
    def cosine_similarity(
        vector_a: np.ndarray,
        vector_b: np.ndarray,
    ) -> float:
        a = np.asarray(vector_a)
        b = np.asarray(vector_b)

        if a.shape != b.shape:
            raise ValueError("Vector dimensions must match")

        denominator = np.linalg.norm(a) * np.linalg.norm(b)

        if denominator == 0:
            raise ValueError("Zero vectors cannot be compared")

        return float(np.dot(a, b) / denominator)

    def find_similar(
        self,
        query: str,
        documents: list[str],
        top_k: int = 3,
    ) -> list[dict]:
        if not documents:
            return []

        if top_k <= 0:
            raise ValueError("top_k must be positive")

        query_vector = self.embed([query])[0]
        document_vectors = self.embed(documents)

        scores = document_vectors @ query_vector

        ranked_indices = np.argsort(scores)[::-1][:top_k]

        return [
            {
                "document": documents[i],
                "similarity": float(scores[i]),
            }
            for i in ranked_indices
        ]
