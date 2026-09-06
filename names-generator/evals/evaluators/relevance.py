import re

STOPWORDS = {
    "a", "an", "the", "for", "and", "or", "to", "of", "with", "that",
    "this", "app", "application", "system", "platform", "tool", "software",
    "website", "web", "using", "build", "make", "create", "personal",
    "help", "helper", "track", "tracker",
}


def _keywords(idea: str) -> list[str]:
    words = re.findall(r"[a-z]+", idea.lower())
    return [w for w in words if len(w) >= 3 and w not in STOPWORDS]


def evaluate(candidates, idea, **_):
    keywords = _keywords(idea)

    if not candidates:
        return {
            "evaluator": "relevance",
            "score": 0.0,
            "passed": False,
            "detail": {"message": "no candidates"},
        }

    if not keywords:
        return {
            "evaluator": "relevance",
            "score": 1.0,
            "passed": True,
            "detail": {"message": "no keywords to match"},
        }

    matched = 0
    per_candidate = []
    for candidate in candidates:
        name = candidate.name.lower()
        hits = [k for k in keywords if k in name]
        per_candidate.append({"name": candidate.name, "matched_keywords": hits})
        matched += bool(hits)

    ratio = matched / len(candidates)

    return {
        "evaluator": "relevance",
        "score": ratio,
        "passed": ratio >= 0.3,
        "detail": {
            "keywords": keywords,
            "matched_candidates": matched,
            "total_candidates": len(candidates),
            "per_candidate": per_candidate,
        },
    }