MAX_NAME_LENGTH = 40


def evaluate(candidates, idea, expected_count=10):
    count_ok = len(candidates) == expected_count
    unique = len({c.name.strip().lower() for c in candidates}) == len(candidates)
    short = all(len(c.name) <= MAX_NAME_LENGTH for c in candidates)

    checks = [count_ok, unique, short]

    return {
        "evaluator": "format",
        "score": sum(checks) / len(checks),
        "passed": all(checks),
        "detail": {
            "count": len(candidates),
            "expected_count": expected_count,
            "count_ok": count_ok,
            "unique": unique,
            "short": short,
        },
    }