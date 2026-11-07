import numpy as np
import pytest

from similarity import EmbeddingEngine


def test_cosine_similarity_identical_vectors():
    vector = np.array([1.0, 0.0, 0.0])

    score = EmbeddingEngine.cosine_similarity(vector, vector)

    assert score == pytest.approx(1.0)


def test_cosine_similarity_orthogonal_vectors():
    a = np.array([1.0, 0.0])
    b = np.array([0.0, 1.0])

    score = EmbeddingEngine.cosine_similarity(a, b)

    assert score == pytest.approx(0.0)


def test_cosine_similarity_dimension_mismatch():
    a = np.array([1.0, 0.0])
    b = np.array([1.0, 0.0, 0.0])

    with pytest.raises(ValueError):
        EmbeddingEngine.cosine_similarity(a, b)


def test_zero_vector_rejected():
    a = np.array([0.0, 0.0])
    b = np.array([1.0, 0.0])

    with pytest.raises(ValueError):
        EmbeddingEngine.cosine_similarity(a, b)


def test_empty_documents():
    assert EmbeddingEngine.find_similar(None, "query", []) == []
