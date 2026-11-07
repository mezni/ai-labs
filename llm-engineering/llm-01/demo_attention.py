import numpy as np

from attention import (
    scaled_dot_product_attention,
)

# Three tokens, each represented by four dimensions.
X = np.array(
    [
        [1.0, 0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0, 1.0],
        [1.0, 1.0, 0.0, 0.0],
    ]
)

# Simplified example: Q, K and V are identical.
output, weights = scaled_dot_product_attention(
    query=X,
    key=X,
    value=X,
    causal=True,
)

print("Attention weights:")
print(weights)

print("\nAttention output:")
print(output)
