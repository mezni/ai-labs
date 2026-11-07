import numpy as np

from sampler import Sampler

vocabulary = [
    "slow",
    "unstable",
    "working",
    "disconnected",
    "excellent",
]

# Synthetic logits from a hypothetical language model.
logits = np.array([3.0, 2.4, 1.8, 1.0, 0.2])

print("Greedy:", vocabulary[Sampler.greedy(logits)])

strategies = [
    {"temperature": 0.3},
    {"temperature": 0.7},
    {"temperature": 1.0},
    {"temperature": 1.0, "top_k": 2},
    {"temperature": 1.0, "top_p": 0.8},
]

for config in strategies:
    token_id = Sampler.sample(
        logits,
        seed=42,
        **config,
    )

    print(config, "->", vocabulary[token_id])
