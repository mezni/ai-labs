import numpy as np


def softmax(x: np.ndarray) -> np.ndarray:
    """Numerically stable softmax."""
    shifted = x - np.max(x, axis=-1, keepdims=True)
    exp_x = np.exp(shifted)
    return exp_x / np.sum(exp_x, axis=-1, keepdims=True)


def scaled_dot_product_attention(
    query: np.ndarray,
    key: np.ndarray,
    value: np.ndarray,
    causal: bool = False,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Calculate scaled dot-product attention.

    Input shape: (sequence_length, embedding_dimension)
    """
    dimension = query.shape[-1]

    scores = query @ key.T / np.sqrt(dimension)

    if causal:
        seq_length = scores.shape[-1]
        mask = np.triu(
            np.ones((seq_length, seq_length), dtype=bool),
            k=1,
        )
        scores = np.where(mask, -np.inf, scores)

    weights = softmax(scores)

    output = weights @ value

    return output, weights
