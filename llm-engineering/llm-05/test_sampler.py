import numpy as np
import pytest

from sampler import Sampler


def test_softmax_probabilities_sum_to_one():
    logits = np.array([2.0, 1.0, 0.5])

    probabilities = Sampler.softmax(logits)

    assert np.isclose(probabilities.sum(), 1.0)


def test_greedy_selects_highest_logit():
    logits = np.array([1.0, 3.0, 2.0])

    assert Sampler.greedy(logits) == 1


def test_top_k_keeps_only_k_candidates():
    logits = np.array([1.0, 4.0, 2.0, 3.0])

    filtered = Sampler.top_k_filter(logits, k=2)

    assert np.isfinite(filtered).sum() == 2
    assert np.isfinite(filtered[1])
    assert np.isfinite(filtered[3])


def test_top_p_keeps_threshold_crossing_token():
    logits = np.log([0.50, 0.30, 0.15, 0.05])

    filtered = Sampler.top_p_filter(logits, p=0.80)

    assert np.isfinite(filtered[0])
    assert np.isfinite(filtered[1])
    assert not np.isfinite(filtered[2])


def test_sampling_is_reproducible_with_same_seed():
    logits = np.array([2.0, 1.5, 1.0, 0.5])

    first = Sampler.sample(logits, seed=42)
    second = Sampler.sample(logits, seed=42)

    assert first == second


def test_temperature_must_be_positive():
    with pytest.raises(ValueError):
        Sampler.softmax(np.array([1.0, 2.0]), temperature=0)


def test_invalid_top_k_is_rejected():
    with pytest.raises(ValueError):
        Sampler.top_k_filter(np.array([1.0, 2.0]), k=3)


def test_repetition_penalty_changes_previous_token_logit():
    logits = np.array([2.0, 1.0, -1.0])

    adjusted = Sampler.apply_repetition_penalty(
        logits,
        generated_tokens=[0, 2],
        penalty=2.0,
    )

    assert adjusted[0] == 1.0
    assert adjusted[2] == -2.0
    assert adjusted[1] == 1.0
