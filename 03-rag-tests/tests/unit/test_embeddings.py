"""Unit tests for EmbeddingGenerator, BM25Index, and Reranker."""

from __future__ import annotations

import httpx
import pytest

from src.knowledge.embeddings import BM25Index, EmbeddingGenerator, Reranker


def _mock(status: int = 200, payload: dict | None = None):
    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(status_code=status, json=payload or {}, request=request)

    return httpx.MockTransport(handler)


def _embedding_payload(items: list[list[float] | None] | None = None, request: httpx.Request | None = None):
    """Build an embeddings response with one entry per input in the request."""
    import json

    n_inputs = len(json.loads(request.content)["input"]) if request is not None else 1
    data = [
        {"embedding": [0.1, 0.2, 0.3] if items and i < len(items) and items[i] is None else (items[i] if items and i < len(items) else [0.1, 0.2, 0.3])}
        for i in range(n_inputs)
    ]
    return {"data": data}


# -- EmbeddingGenerator -------------------------------------------------------


def test_embedding_init_reads_config(env_openrouter, patch_knowledge_config):
    gen = EmbeddingGenerator(transport=_mock())
    assert gen.model == "openai/text-embedding-3-small"
    assert gen.batch_size == 64
    assert gen.url == "https://openrouter.ai/api/v1/embeddings"
    assert gen.dimensions is None
    gen.close()


def test_embedding_requires_api_key(monkeypatch, patch_knowledge_config):
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)
    with pytest.raises(RuntimeError, match="OPENROUTER_API_KEY"):
        EmbeddingGenerator(transport=_mock())


def test_embedding_embed_text(env_openrouter, patch_knowledge_config):
    gen = EmbeddingGenerator(transport=_mock(200, _embedding_payload()))
    assert gen.embed_text("hello") == [0.1, 0.2, 0.3]
    assert gen.dimensions == 3
    gen.close()


def test_embedding_batches_requests(env_openrouter, patch_llm_config, monkeypatch):
    import src.knowledge.embeddings as embeddings

    calls: list[list[str]] = []

    def handler(request: httpx.Request) -> httpx.Response:
        import json

        calls.append(json.loads(request.content)["input"])
        return httpx.Response(
            200, json=_embedding_payload(request=request), request=request
        )

    monkeypatch.setattr(
        embeddings,
        "load_config",
        lambda: {
            "models": {
                "embedding": {
                    "model": "m",
                    "openrouter_url": "https://openrouter.ai/api/v1/embeddings",
                    "batch_size": 2,
                    "timeout_seconds": 10,
                }
            }
        },
    )
    gen = EmbeddingGenerator(transport=httpx.MockTransport(handler))
    vectors = gen.embed_documents(["a", "b", "c", "d", "e"])
    assert len(vectors) == 5
    assert [len(batch) for batch in calls] == [2, 2, 1]
    assert gen.dimensions == 3
    gen.close()


def test_embedding_empty_list_returns_empty(env_openrouter, patch_knowledge_config):
    calls = []

    def handler(request: httpx.Request) -> httpx.Response:
        calls.append(request)
        return httpx.Response(200, json={"data": []}, request=request)

    gen = EmbeddingGenerator(transport=httpx.MockTransport(handler))
    assert gen.embed_documents([]) == []
    assert calls == []
    gen.close()


def test_embedding_sends_auth_header(env_openrouter, patch_knowledge_config):
    captured = {}

    def handler(request: httpx.Request) -> httpx.Response:
        captured["auth"] = request.headers.get("Authorization")
        return httpx.Response(
            200, json=_embedding_payload(request=request), request=request
        )

    gen = EmbeddingGenerator(transport=httpx.MockTransport(handler))
    gen.embed_text("x")
    assert captured["auth"] == "Bearer test-openrouter-key"
    gen.close()


def test_embedding_http_error_raises(env_openrouter, patch_knowledge_config):
    gen = EmbeddingGenerator(transport=_mock(status=429))
    with pytest.raises(httpx.HTTPStatusError):
        gen.embed_documents(["x"])
    gen.close()


# -- BM25Index ----------------------------------------------------------------


DOCS = [
    "annual leave policy allows thirty days",
    "access control requires manager approval",
    "data backup completes every night",
]


def test_bm25_search_ranks_by_relevance():
    index = BM25Index().add_documents(DOCS)
    results = index.search("how many leave days", top_k=2)
    assert results
    assert results[0]["index"] == 0
    assert results[0]["text"] == DOCS[0]
    assert results[0]["score"] > 0


def test_bm25_search_honors_top_k():
    index = BM25Index().add_documents(DOCS)
    assert len(index.search("leave access", top_k=1)) == 1


def test_bm25_search_drops_zero_score_hits():
    index = BM25Index().add_documents(DOCS)
    assert index.search("quantum physics", top_k=5) == []


def test_bm25_get_scores_length():
    index = BM25Index().add_documents(DOCS)
    scores = index.get_scores("leave")
    assert len(scores) == len(DOCS)


def test_bm25_search_before_build_raises():
    index = BM25Index()
    with pytest.raises(RuntimeError, match="add_documents"):
        index.search("leave")


def test_bm25_len_and_has_documents():
    index = BM25Index()
    assert len(index) == 0
    assert not index.has_documents()
    index.add_documents(DOCS)
    assert len(index) == 3
    assert index.has_documents()


def test_bm25_tokenize():
    assert BM25Index.tokenize("Annual Leave! POLICY-2024") == [
        "annual",
        "leave",
        "policy",
        "2024",
    ]


def test_bm25_custom_tokenizer():
    index = BM25Index(tokenizer=lambda t: t.split()).add_documents(DOCS)
    assert index.has_documents()
    assert len(index.search("annual leave policy allows thirty days", top_k=1)) == 1


# -- Reranker -----------------------------------------------------------------

RERANK_PAYLOAD = {
    "results": [
        {"index": 1, "relevance_score": 0.9},
        {"index": 0, "relevance_score": 0.2},
    ]
}


def test_rerank_returns_reordered_documents(env_openrouter, patch_knowledge_config):
    reranker = Reranker(transport=_mock(200, RERANK_PAYLOAD))
    assert reranker.rerank("q", ["a", "b"]) == ["b", "a"]
    reranker.close()


def test_rerank_with_scores_sorted(env_openrouter, patch_knowledge_config):
    reranker = Reranker(transport=_mock(200, RERANK_PAYLOAD))
    assert reranker.rerank_with_scores("q", ["a", "b"]) == [("b", 0.9), ("a", 0.2)]
    reranker.close()


def test_rerank_empty_documents(env_openrouter, patch_knowledge_config):
    calls = []

    def handler(request: httpx.Request) -> httpx.Response:
        calls.append(request)
        return httpx.Response(200, json={"results": []}, request=request)

    reranker = Reranker(transport=httpx.MockTransport(handler))
    assert reranker.rerank("q", []) == []
    assert calls == []
    reranker.close()


def test_rerank_top_n_default_from_config(env_openrouter, patch_knowledge_config):
    captured = {}

    def handler(request: httpx.Request) -> httpx.Response:
        import json

        captured["json"] = json.loads(request.content)
        return httpx.Response(
            200, json={"results": [{"index": 0, "relevance_score": 0.5}]}, request=request
        )

    reranker = Reranker(transport=httpx.MockTransport(handler))
    reranker.rerank("q", ["a"])
    assert captured["json"]["top_n"] == 5
    assert captured["json"]["model"] == "cohere/rerank-v3.5"
    reranker.close()


def test_rerank_requires_api_key(monkeypatch, patch_knowledge_config):
    monkeypatch.delenv("OPENROUTER_API_KEY", raising=False)
    with pytest.raises(RuntimeError, match="OPENROUTER_API_KEY"):
        Reranker(transport=_mock())


def test_rerank_http_error_raises(env_openrouter, patch_knowledge_config):
    reranker = Reranker(transport=_mock(status=500))
    with pytest.raises(httpx.HTTPStatusError):
        reranker.rerank("q", ["a"])
    reranker.close()