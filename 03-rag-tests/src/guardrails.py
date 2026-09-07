"""Guardrails for the RAG pipeline.

Three stages, applied in order before/after generation:

1. Input guardrail  — reject empty or whitespace-only questions.
2. Retrieval guardrail — when the top chunk's similarity to the query is too
   low, refuse to answer: "No relevant information was found."
3. Generation grounding guardrail — after generation, if the answer is not
   lexically grounded in the retrieved context, refuse it as ungrounded
   (prevents hallucinations from reaching the user).

Thresholds can be overridden via config/llm_config.yaml::

    guardrails:
      retrieval:
        min_similarity: 0.30
      generation:
        min_groundedness: 0.30
"""

from __future__ import annotations

from typing import Any

from src.evaluation.metrics import lexical_groundedness
from src.utils import get_logger, load_config

logger = get_logger(__name__)

EMPTY_QUESTION_MESSAGE = "Question must not be empty."
NO_RELEVANT_MESSAGE = "No relevant information was found."
UNGROUNDED_MESSAGE = (
    "I couldn't find a grounded answer in the retrieved information."
)

DEFAULT_MIN_SIMILARITY = 0.30
DEFAULT_MIN_GROUNDEDNESS = 0.30


class GuardrailError(ValueError):
    """Raised when an input violates a guardrail."""


class EmptyQuestionError(GuardrailError):
    """Raised by the input guardrail for empty/whitespace-only questions."""


def _setting(section: str, key: str, default: float) -> float:
    try:
        config = load_config()
        value = config.get("guardrails", {}).get(section, {}).get(key)
        if value is None:
            return default
        return float(value)
    except Exception:  # pragma: no cover - config is optional
        return default


# -- input guardrail ----------------------------------------------------------


def validate_query(question: str | None) -> str:
    """Input guardrail: reject empty questions and return the trimmed question.

    Raises EmptyQuestionError when the question is missing or blank.
    """
    if question is None or not str(question).strip():
        logger.warning("Input guardrail rejected an empty question")
        raise EmptyQuestionError(EMPTY_QUESTION_MESSAGE)
    return str(question).strip()


# -- retrieval guardrail ------------------------------------------------------


def check_retrieval(
    chunks: list[dict[str, Any]], min_similarity: float | None = None
) -> str | None:
    """Retrieval guardrail: returns a refusal message when nothing is relevant.

    Uses the top chunk's dense (cosine) similarity against the query. Returns
    None when retrieval is strong enough to proceed.
    """
    threshold = _setting("retrieval", "min_similarity", DEFAULT_MIN_SIMILARITY)
    if min_similarity is not None:
        threshold = min_similarity

    if not chunks:
        logger.info("Retrieval guardrail: no chunks retrieved")
        return NO_RELEVANT_MESSAGE

    top = chunks[0]
    similarity = top.get("dense_similarity")
    if similarity is None:
        similarity = top.get("score")
    if similarity is None or similarity < threshold:
        logger.info(
            "Retrieval guardrail rejected top similarity %.3f (< %.2f)",
            similarity,
            threshold,
        )
        return NO_RELEVANT_MESSAGE
    return None


# -- generation grounding guardrail -------------------------------------------


def check_grounding(
    answer: str,
    context: str,
    min_groundedness: float | None = None,
    min_overlap: float = 0.3,
) -> str | None:
    """Grounding guardrail: refuses an answer with too little context support.

    Uses lexical_groundedness (mean support of the answer's sentences in the
    retrieved context). Returns a refusal message when ungrounded, else None.
    """
    threshold = _setting("generation", "min_groundedness", DEFAULT_MIN_GROUNDEDNESS)
    if min_groundedness is not None:
        threshold = min_groundedness

    groundedness = lexical_groundedness(answer, context, min_overlap)
    if groundedness < threshold:
        logger.info(
            "Grounding guardrail rejected answer (groundedness=%.3f < %.2f)",
            groundedness,
            threshold,
        )
        return UNGROUNDED_MESSAGE
    return None