import numpy as np

from attention import (
    scaled_dot_product_attention,
)


def test_attention_output_shape():
    x = np.random.default_rng(42).random((3, 4))

    output, weights = scaled_dot_product_attention(x, x, x)

    assert output.shape == (3, 4)
    assert weights.shape == (3, 3)


def test_attention_weights_sum_to_one():
    x = np.random.default_rng(42).random((3, 4))

    _, weights = scaled_dot_product_attention(x, x, x)

    np.testing.assert_allclose(
        weights.sum(axis=-1),
        np.ones(3),
    )


def test_causal_mask_blocks_future_tokens():
    x = np.random.default_rng(42).random((4, 4))

    _, weights = scaled_dot_product_attention(x, x, x, causal=True)

    assert weights[0, 1] == 0
    assert weights[0, 2] == 0
    assert weights[1, 3] == 0
