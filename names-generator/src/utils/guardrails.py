import re

PROMPT_INJECTION_PATTERNS = [
    r"ignore\s+(all\s+|your\s+|the\s+)?(previous\s+)?(instructions|prompt)",
    r"(reveal|show|print)\s+(your|the)\s+(system\s+)?prompt",
    r"disregard\s+(all\s+)?(previous\s+)?instructions",
    r"forget\s+(everything|all|your instructions)",
    r"jailbreak",
    r"bypass\s+(the\s+)?(rules|safety|guardrails)",
    r"you\s+are\s+now",
]

BLOCKED_NAME_TERMS = [
    "admin",
    "administrator",
    "test",
    "dummy",
    "placeholder",
    "untitled",
    "temp",
    "default",
    "example",
    "lorem",
    "xoxo",
    "n/a",
]

MAX_NAME_LENGTH = 40

_CONTROL_CHARS = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]")


class InputGuardrailViolation(ValueError):
    pass


class OutputGuardrailViolation(ValueError):
    pass


def validate_input_idea(idea: str, max_length: int = 1000) -> str:
    if idea is None or not idea.strip():
        raise InputGuardrailViolation("Empty input")

    text = idea.strip()
    if len(text) > max_length:
        raise InputGuardrailViolation(
            f"Idea too long: {len(text)} characters (max {max_length})"
        )

    if _CONTROL_CHARS.search(text):
        raise InputGuardrailViolation("Input contains invalid control characters")

    lowered = text.lower()
    for pattern in PROMPT_INJECTION_PATTERNS:
        if re.search(pattern, lowered):
            raise InputGuardrailViolation("Suspicious input detected")

    return text


def validate_output_names(names, expected: int):
    if not names:
        raise OutputGuardrailViolation("No names generated")

    if len(names) != expected:
        raise OutputGuardrailViolation(
            f"Expected exactly {expected} names, got {len(names)}"
        )

    seen = set()
    for candidate in names:
        if not candidate.name or not candidate.description or not candidate.reason:
            raise OutputGuardrailViolation("Candidate missing required fields")

        if len(candidate.name) > MAX_NAME_LENGTH:
            raise OutputGuardrailViolation(
                f"Name too long ({len(candidate.name)} > {MAX_NAME_LENGTH}): {candidate.name}"
            )

        lowered = candidate.name.lower()
        if lowered in seen:
            raise OutputGuardrailViolation(f"Duplicate name: {candidate.name}")
        seen.add(lowered)

        if any(term in lowered for term in BLOCKED_NAME_TERMS):
            raise OutputGuardrailViolation(f"Name uses prohibited term: {candidate.name}")

    return names