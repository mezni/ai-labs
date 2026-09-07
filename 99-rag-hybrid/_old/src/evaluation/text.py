"""Small text helpers for evaluation output parsing."""
from __future__ import annotations

import json
import re
from typing import Any


def extract_json(text: str) -> dict[str, Any] | None:
    """Extract the first JSON object from model output.

    Tolerates prose around the JSON and ```json ... ``` fences.
    """
    if not text:
        return None
    stripped = text.strip()
    if stripped.startswith("```"):
        stripped = re.sub(r"^```(?:json)?\s*|\s*```$", "", stripped).strip()
    try:
        return json.loads(stripped)
    except json.JSONDecodeError:
        pass
    match = re.search(r"\{.*\}", stripped, re.DOTALL)
    if not match:
        return None
    try:
        return json.loads(match.group(0))
    except json.JSONDecodeError:
        return None