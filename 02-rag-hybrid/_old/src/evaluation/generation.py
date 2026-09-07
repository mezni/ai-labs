"""Generation evaluation: correctness, groundedness (faithfulness), hallucination.

Uses the pipeline's LLM (via `Generator`) as a judge to score a candidate
answer against both the reference answer and the retrieved context that the
answer was generated from.
"""
from __future__ import annotations

from typing import TYPE_CHECKING, Any

from app.main import build_generation_prompt
from src.evaluation.text import extract_json

if TYPE_CHECKING:  # pragma: no cover
    from app.main import Generator
    from src.knowledge.retriever import Retriever

JUDGE_SYSTEM_PROMPT = (
    "You evaluate AI answers that were generated with retrieval-augmented generation. "
    "You are given the user's question, the retrieved context chunks, a reference "
    "answer, and the candidate answer. Score the candidate answer and detect "
    "hallucination. Rules: faithfulness_score measures whether every claim in the "
    "candidate answer is supported by the retrieved context (a claim is a "
    "hallucination when it is unsupported by the context). correctness_score measures "
    "how well the candidate matches the reference answer. hallucination is true when "
    "the candidate contains any unsupported claim. Reply with ONLY a JSON object: "
    '{"correctness_score": int 0-5, "faithfulness_score": int 0-5, '
    '"hallucination": bool, "explanation": str}.'
)

JUDGE_USER_TEMPLATE = (
    "QUESTION:\n{question}\n\n"
    "REFERENCE ANSWER:\n{reference_answer}\n\n"
    "RETRIEVED CONTEXT:\n{retrieved_context}\n\n"
    "CANDIDATE ANSWER:\n{candidate_answer}"
)

DEFAULT_VERDICT: dict[str, Any] = {
    "correctness_score": 0.0,
    "faithfulness_score": 0.0,
    "hallucination": True,
    "explanation": "Judge output could not be parsed",
}


def build_judge_prompt(
    question: str,
    answer: str,
    context: list[dict[str, Any]],
    expected_answer: str,
) -> list[dict[str, str]]:
    """Build the judge chat messages (system + user)."""
    retrieved_context = "\n\n".join(
        f"[source: {chunk.get('source', '?')}]\n{chunk.get('content', '')}"
        for chunk in context
    )
    user_prompt = JUDGE_USER_TEMPLATE.format(
        question=question,
        reference_answer=expected_answer,
        retrieved_context=retrieved_context,
        candidate_answer=answer,
    )
    return [
        {"role": "system", "content": JUDGE_SYSTEM_PROMPT},
        {"role": "user", "content": user_prompt},
    ]


def _coerce_verdict(parsed: dict[str, Any]) -> dict[str, Any]:
    def _score(value: Any) -> float:
        try:
            return max(0.0, min(5.0, float(value)))
        except (TypeError, ValueError):
            return 0.0

    verdict = dict(DEFAULT_VERDICT)
    verdict["correctness_score"] = _score(parsed.get("correctness_score")) / 5.0
    verdict["faithfulness_score"] = _score(parsed.get("faithfulness_score")) / 5.0
    hallucination = parsed.get("hallucination")
    verdict["hallucination"] = bool(hallucination)
    verdict["explanation"] = str(parsed.get("explanation", ""))
    return verdict


class GenerationJudge:
    """LLM-as-judge scoring candidate answers for the generation stage."""

    def __init__(self, generator: "Generator") -> None:
        self._generator = generator

    def judge(
        self,
        question: str,
        answer: str,
        context: list[dict[str, Any]],
        expected_answer: str,
    ) -> dict[str, Any]:
        """Return the verdict dict for a single candidate answer."""
        messages = build_judge_prompt(question, answer, context, expected_answer)
        raw = self._generator.generate(messages)
        parsed = extract_json(raw)
        if parsed is None:
            return dict(DEFAULT_VERDICT)
        return _coerce_verdict(parsed)


def evaluate_generation(
    retriever: "Retriever",
    generator: "Generator",
    dataset: list[dict[str, Any]],
    top_k: int = 5,
) -> dict[str, Any]:
    """Run the full pipeline per question, then judge each answer."""
    judge = GenerationJudge(generator)
    per_query: list[dict[str, Any]] = []
    for entry in dataset:
        question = entry["question"]
        context = retriever.retrieve(question, top_k=top_k)
        answer = generator.generate(
            build_generation_prompt(question, context)
        )
        verdict = judge.judge(question, answer, context, entry["expected_answer"])
        per_query.append(
            {
                "question": question,
                "answer": answer,
                "retrieved_sources": sorted(
                    {chunk.get("source", "?") for chunk in context}
                ),
                **verdict,
            }
        )

    def _mean(field: str) -> float:
        if not per_query:
            return 0.0
        return sum(row[field] for row in per_query) / len(per_query)

    return {
        "num_queries": len(per_query),
        "metrics": {
            "mean_correctness": _mean("correctness_score"),
            "mean_faithfulness": _mean("faithfulness_score"),
            "hallucination_rate": _mean("hallucination"),
        },
        "per_query": per_query,
    }