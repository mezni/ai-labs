def _levenshtein(a: str, b: str) -> int:
    if len(a) < len(b):
        a, b = b, a

    if not b:
        return len(a)

    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        curr = [i]
        for j, cb in enumerate(b, 1):
            cost = 0 if ca == cb else 1
            curr.append(min(curr[-1] + 1, prev[j] + 1, prev[j - 1] + cost))
        prev = curr
    return prev[-1]


def evaluate(candidates, idea, **_):
    if len(candidates) < 2:
        return {
            "evaluator": "diversity",
            "score": 1.0,
            "passed": True,
            "detail": {"message": "too few candidates to measure diversity"},
        }

    lower = [c.name.strip().lower() for c in candidates]

    unique_ratio = len(set(lower)) / len(lower)

    total = 0
    pairs = 0
    for i in range(len(lower)):
        for j in range(i + 1, len(lower)):
            denom = max(len(lower[i]), len(lower[j]), 1)
            total += 1 - (_levenshtein(lower[i], lower[j]) / denom)
            pairs += 1
    similarity = total / pairs
    edit_distance_score = 1 - similarity

    score = 0.5 * unique_ratio + 0.5 * edit_distance_score

    return {
        "evaluator": "diversity",
        "score": score,
        "passed": score >= 0.5,
        "detail": {
            "unique_ratio": unique_ratio,
            "pairwise_similarity": similarity,
        },
    }