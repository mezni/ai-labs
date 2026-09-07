"""Generation evaluation: was the answer correct, grounded, hallucination-free?"""

from __future__ import annotations

import json
import re
from typing import Any

from src.evaluation import metrics
from src.knowledge.embeddings import EmbeddingGenerator
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.utils import get_logger

logger = get_logger(__name__)

EMBEDDING_SIMILARITY_THRESHOLD = 0.70

JUDGE_PROMPT = "generation_eval"


def extract_json(text: str) -> dict[str, Any]:
    """Extract the first JSON object from a model response."""
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        match = re.search(r"\{[\s\S]*\}", text)
        if not match:
            raise ValueError(f"No JSON object found in response: {text[:200]}")
        return json.loads(match.group(0))


def answer_embedding_similarity(
    embedder: EmbeddingGenerator,
    answer: str,
    reference: str,
) -> float:
    """Cosine similarity between the answer and the reference embedding."""
    vectors = embedder.embed_documents([answer, reference])
    return metrics.cosine_similarity(vectors[0], vectors[1])


def judge_answer(
    client: LLMClient,
    prompt_manager: PromptManager,
    question: str,
    answer: str,
    reference: str,
    context: str,
) -> dict[str, Any]:
    """LLM-as-judge verdict: correctness, groundedness, hallucination."""
    prompt = prompt_manager.get_prompt(JUDGE_PROMPT)
    user = prompt_manager.format_prompt(
        prompt["user_template"],
        question=question,
        answer=answer,
        expected_answer=reference,
        context=context,
    )
    messages: list[dict[str, str]] = []
    if prompt.get("system"):
        messages.append({"role": "system", "content": prompt["system"]})
    messages.append({"role": "user", "content": user})
    return extract_json(client.generate(messages))


def _context_text(chunks: list[dict[str, Any]]) -> str:
    return "\n\n".join(f"[{c.get('source', '')}]\n{c['content']}" for c in chunks)


def evaluate_generation(
    items: list[dict[str, Any]],
    embedder: EmbeddingGenerator,
    similarity_threshold: float = EMBEDDING_SIMILARITY_THRESHOLD,
    use_llm_judge: bool = False,
    client: LLMClient | None = None,
    prompt_manager: PromptManager | None = None,
) -> dict[str, Any]:
    """Evaluate answers for correctness, groundedness, and hallucination.

    Each item: {question, answer, expected_answer, context_chunks}.
    Context chunks are dicts with ``content`` (and optional ``source``).

    ``use_llm_judge`` gates the LLM-as-judge pass (requires client + manager);
    the embedding-similarity and lexical checks always run.
    """
    rows: list[dict[str, Any]] = []
    for i, item in enumerate(items, start=1):
        question = item["question"]
        answer = item["answer"]
        reference = item["expected_answer"]
        context = _context_text(item.get("context_chunks", []))

        similarity = answer_embedding_similarity(embedder, answer, reference)
        grounded = metrics.lexical_groundedness(answer, context)
        hallucinated = metrics.hallucination_ratio(answer, context)

        row: dict[str, Any] = {
            "index": i,
            "question": question,
            "answer": answer,
            "expected_answer": reference,
            "similarity": similarity,
            "correct_lexical": similarity >= similarity_threshold,
            "groundedness": grounded,
            "hallucination_ratio": hallucinated,
        }

        if use_llm_judge:
            if client is None or prompt_manager is None:
                raise ValueError("client and prompt_manager are required for LLM judge")
            verdict = judge_answer(client, prompt_manager, question, answer, reference, context)
            row["judge"] = verdict
            row["correct_judge"] = float(verdict.get("correctness", 0.0)) >= 0.5
            row["grounded_judge"] = bool(verdict.get("grounded", False))
            row["hallucinated_judge"] = bool(verdict.get("hallucinated", False))
        rows.append(row)

    aggregate: dict[str, Any] = {
        "num_questions": len(rows),
        "correctness_lexical": metrics.mean(
            [float(r["correct_lexical"]) for r in rows]
        ),
        "mean_similarity": metrics.mean([r["similarity"] for r in rows]),
        "groundedness": metrics.mean([r["groundedness"] for r in rows]),
        "hallucination_ratio": metrics.mean([r["hallucination_ratio"] for r in rows]),
    }
    if use_llm_judge:
        aggregate["correctness_judge"] = metrics.mean(
            [float(r["correct_judge"]) for r in rows]
        )
        aggregate["groundedness_judge"] = metrics.mean(
            [float(r["grounded_judge"]) for r in rows]
        )
        aggregate["hallucination_rate_judge"] = metrics.mean(
            [float(r["hallucinated_judge"]) for r in rows]
        )
    aggregate["per_question"] = rows
    return aggregate