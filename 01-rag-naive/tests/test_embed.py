"""Tests for ``rag.embed`` OpenRouter embedding generation."""

from types import SimpleNamespace

import pytest
from pydantic import ValidationError

from rag.config import Settings
from rag.embed import Embedder, Embedding, EmbeddingError, build_embedder, validate_dimension


def _vector(dimension=3, seed=0.1):
    return [round(seed + offset, 3) for offset in range(dimension)]


def _response(vectors):
    data = [SimpleNamespace(index=i, embedding=vector) for i, vector in enumerate(vectors)]
    return SimpleNamespace(data=data)


class StubEmbeddings:
    def __init__(self, responder):
        self.responder = responder
        self.calls = []

    def create(self, input, model):
        self.calls.append({"input": list(input), "model": model})
        return self.responder(self.calls[-1])


class StubClient:
    def __init__(self, responder):
        self.embeddings = StubEmbeddings(responder)


def _embedder(client, model="openai/text-embedding-3-small", **kwargs):
    return Embedder(client, model=model, **kwargs)


def test_embed_texts_returns_ordered_embeddings():
    client = StubClient(lambda call: _response([_vector(), _vector(seed=0.9)]))
    embedder = _embedder(client)

    embeddings = embedder.embed_texts(["refund policy", "roaming charges"])

    assert [e.index for e in embeddings] == [0, 1]
    assert [e.text for e in embeddings] == ["refund policy", "roaming charges"]
    assert [e.vector for e in embeddings] == [_vector(), _vector(seed=0.9)]


def test_embed_texts_maps_texts_by_response_index():
    data = [
        SimpleNamespace(index=1, embedding=_vector(seed=0.9)),
        SimpleNamespace(index=0, embedding=_vector(seed=0.1)),
    ]
    client = StubClient(lambda call: SimpleNamespace(data=data))
    embedder = _embedder(client)

    embeddings = embedder.embed_texts(["refund policy", "roaming charges"])

    assert [(e.index, e.text, e.vector) for e in embeddings] == [
        (1, "roaming charges", _vector(seed=0.9)),
        (0, "refund policy", _vector(seed=0.1)),
    ]


def test_embed_texts_sends_model_and_inputs():
    client = StubClient(lambda call: _response([_vector()]))
    embedder = _embedder(client, model="custom/embed-model")

    embedder.embed_texts(["hello"])

    assert len(client.embeddings.calls) == 1
    assert client.embeddings.calls[0]["model"] == "custom/embed-model"
    assert client.embeddings.calls[0]["input"] == ["hello"]


def test_embed_texts_empty_list_returns_empty():
    client = StubClient(lambda call: pytest.fail("no request expected"))
    embedder = _embedder(client)

    assert embedder.embed_texts([]) == []
    assert client.embeddings.calls == []


def test_embed_text_returns_single_embedding():
    client = StubClient(lambda call: _response([_vector()]))
    embedder = _embedder(client)

    embedding = embedder.embed_text("refund policy")

    assert embedding.index == 0
    assert embedding.text == "refund policy"
    assert embedding.vector == _vector()


class FlakyResponder:
    def __init__(self, failures, vectors):
        self.failures = failures
        self.attempts = 0
        self.vectors = vectors

    def __call__(self, call):
        self.attempts += 1
        if self.attempts <= self.failures:
            raise RuntimeError("transient failure")
        return _response(self.vectors)


def test_embed_texts_retries_transient_failures(monkeypatch):
    sleeps = []
    monkeypatch.setattr("rag.embed.time.sleep", sleeps.append)
    responder = FlakyResponder(failures=2, vectors=[_vector()])
    client = StubClient(responder)
    embedder = _embedder(client, max_retries=3)

    embeddings = embedder.embed_texts(["refund policy"])

    assert responder.attempts == 3
    assert len(sleeps) == 2
    assert len(embeddings) == 1


def test_embed_texts_raises_after_exhausting_retries(monkeypatch):
    monkeypatch.setattr("rag.embed.time.sleep", lambda seconds: None)
    responder = FlakyResponder(failures=99, vectors=[_vector()])
    client = StubClient(responder)
    embedder = _embedder(client, max_retries=2)

    with pytest.raises(EmbeddingError, match="3 attempts"):
        embedder.embed_texts(["refund policy"])

    assert responder.attempts == 3


def test_embed_texts_accepts_matching_dimension():
    client = StubClient(lambda call: _response([_vector(dimension=3)]))
    embedder = _embedder(client)

    embeddings = embedder.embed_texts(["refund policy"], expected_dimension=3)

    assert embeddings[0].vector == _vector(dimension=3)


def test_embed_texts_rejects_dimension_mismatch():
    client = StubClient(lambda call: _response([_vector(dimension=3)]))
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="1536"):
        embedder.embed_texts(["refund policy"], expected_dimension=1536)


def test_embed_texts_rejects_mixed_dimensions():
    client = StubClient(lambda call: _response([_vector(dimension=3), _vector(dimension=4)]))
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="mixed dimensions"):
        embedder.embed_texts(["a", "b"])


def test_embed_texts_raises_when_response_has_no_data():
    client = StubClient(lambda call: SimpleNamespace())
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="data list"):
        embedder.embed_texts(["refund policy"])


def test_embed_texts_raises_on_response_count_mismatch():
    client = StubClient(lambda call: _response([_vector()]))
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="1 embeddings for 2 inputs"):
        embedder.embed_texts(["a", "b"])


def test_embed_texts_raises_on_invalid_vector():
    client = StubClient(
        lambda call: SimpleNamespace(data=[SimpleNamespace(index=0, embedding=["not-float"])])
    )
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="invalid vector"):
        embedder.embed_texts(["refund policy"])


def test_embed_texts_raises_on_invalid_response_index():
    client = StubClient(
        lambda call: SimpleNamespace(data=[SimpleNamespace(index=7, embedding=[0.1])])
    )
    embedder = _embedder(client)

    with pytest.raises(EmbeddingError, match="invalid index"):
        embedder.embed_texts(["refund policy"])


def test_validate_dimension_accepts_matching_dimension():
    embedding = Embedding(index=0, text="t", vector=[0.1, 0.2, 0.3])

    validate_dimension([embedding], 3)


def test_validate_dimension_raises_on_mismatch():
    embedding = Embedding(index=0, text="t", vector=[0.1, 0.2, 0.3])

    with pytest.raises(EmbeddingError, match="expected dimension 1536"):
        validate_dimension([embedding], 1536)


def test_validate_dimension_rejects_non_positive_expected():
    embedding = Embedding(index=0, text="t", vector=[0.1])

    with pytest.raises(ValueError, match="expected_dimension"):
        validate_dimension([embedding], 0)


def test_embedder_requires_non_empty_model():
    with pytest.raises(ValueError, match="model"):
        Embedder(StubClient(lambda call: None), model="")


def test_embedder_rejects_negative_retries():
    with pytest.raises(ValueError, match="max_retries"):
        Embedder(StubClient(lambda call: None), model="m", max_retries=-1)


def test_embedder_rejects_negative_retry_delay():
    with pytest.raises(ValueError, match="retry_delay"):
        Embedder(StubClient(lambda call: None), model="m", retry_delay=-1)


def test_build_embedder_uses_openrouter_settings(monkeypatch):
    captured = {}

    class StubOpenAI:
        def __init__(self, **kwargs):
            captured.update(kwargs)

    monkeypatch.setattr("rag.embed.OpenAI", StubOpenAI)
    settings = Settings(
        openrouter_base_url="https://openrouter.example.com",
        openrouter_api_key="sk-test",
        openrouter_embedding_model="openai/text-embedding-3-small",
    )

    embedder = build_embedder(settings)

    assert captured["base_url"] == "https://openrouter.example.com"
    assert captured["api_key"] == "sk-test"
    assert embedder.model == "openai/text-embedding-3-small"


def test_build_embedder_requires_embedding_model():
    with pytest.raises(ValueError, match="OPENROUTER_EMBEDDING_MODEL"):
        build_embedder(Settings())


def test_embedding_model_dump_shape():
    embedding = Embedding(index=0, text="t", vector=[0.1])

    data = embedding.model_dump(mode="json")

    assert set(data) == {"index", "text", "vector"}
    assert data["index"] == 0
    assert data["text"] == "t"
    assert data["vector"] == [0.1]


def test_embedding_rejects_negative_index():
    with pytest.raises(ValidationError):
        Embedding(index=-1, text="t", vector=[0.1])
