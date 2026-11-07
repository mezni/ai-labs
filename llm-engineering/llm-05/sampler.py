import numpy as np


class Sampler:
    @staticmethod
    def _validate_logits(logits: np.ndarray) -> np.ndarray:
        values = np.asarray(logits, dtype=np.float64)

        if values.ndim != 1 or values.size == 0:
            raise ValueError("Logits must be a non-empty 1D array.")

        if not np.all(np.isfinite(values)):
            raise ValueError("Logits must contain only finite values.")

        return values.copy()

    @staticmethod
    def softmax(
        logits: np.ndarray,
        temperature: float = 1.0,
    ) -> np.ndarray:
        values = Sampler._validate_logits(logits)

        if temperature <= 0:
            raise ValueError("Temperature must be greater than zero.")

        scaled = values / temperature
        shifted = scaled - np.max(scaled)

        exp_values = np.exp(shifted)
        return exp_values / np.sum(exp_values)

    @staticmethod
    def greedy(logits: np.ndarray) -> int:
        values = Sampler._validate_logits(logits)
        return int(np.argmax(values))

    @staticmethod
    def apply_repetition_penalty(
        logits: np.ndarray,
        generated_tokens: list[int],
        penalty: float = 1.2,
    ) -> np.ndarray:
        values = Sampler._validate_logits(logits)

        if penalty < 1.0:
            raise ValueError("Penalty must be at least 1.0.")

        for token_id in set(generated_tokens):
            if token_id < 0 or token_id >= len(values):
                raise ValueError("Token ID is outside the vocabulary.")

            if values[token_id] > 0:
                values[token_id] /= penalty
            else:
                values[token_id] *= penalty

        return values

    @staticmethod
    def top_k_filter(
        logits: np.ndarray,
        k: int,
    ) -> np.ndarray:
        values = Sampler._validate_logits(logits)

        if not 1 <= k <= len(values):
            raise ValueError("k must be between 1 and vocabulary size.")

        keep = np.argpartition(values, -k)[-k:]

        filtered = np.full_like(values, -np.inf)
        filtered[keep] = values[keep]

        return filtered

    @staticmethod
    def top_p_filter(
        logits: np.ndarray,
        p: float,
        temperature: float = 1.0,
    ) -> np.ndarray:
        values = Sampler._validate_logits(logits)

        if not 0 < p <= 1:
            raise ValueError("p must be in the interval (0, 1].")

        probabilities = Sampler.softmax(values, temperature)
        sorted_indices = np.argsort(probabilities)[::-1]
        sorted_probs = probabilities[sorted_indices]

        cumulative = np.cumsum(sorted_probs)

        # Keep the token that crosses the probability threshold.
        remove_sorted = cumulative - sorted_probs >= p
        remove_indices = sorted_indices[remove_sorted]

        filtered = values.copy()
        filtered[remove_indices] = -np.inf

        return filtered

    @staticmethod
    def sample(
        logits: np.ndarray,
        temperature: float = 1.0,
        top_k: int | None = None,
        top_p: float | None = None,
        seed: int | None = None,
    ) -> int:
        values = Sampler._validate_logits(logits)

        if temperature <= 0:
            raise ValueError("Use greedy() for deterministic decoding.")

        if top_k is not None:
            values = Sampler.top_k_filter(values, top_k)

        if top_p is not None:
            values = Sampler.top_p_filter(
                values,
                top_p,
                temperature=temperature,
            )

        probabilities = Sampler.softmax(values, temperature)

        rng = np.random.default_rng(seed)
        return int(rng.choice(len(probabilities), p=probabilities))
