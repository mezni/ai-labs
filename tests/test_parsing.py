import json
from pathlib import Path

import pytest

from parsing import UnsupportedContentType, parse

DATA_DIR = Path(__file__).resolve().parent.parent / "data"


def test_markdown_splits_headings_and_clauses() -> None:
    data = (DATA_DIR / "raw/billing/AW-BIL-001_billing_dispute_policy.md").read_bytes()
    blocks = parse(data, "text/markdown")
    headings = [b for b in blocks if b.kind == "heading"]
    clauses = [b for b in blocks if b.kind == "paragraph"]
    assert len(headings) == 5
    assert len(clauses) == 7
    assert headings[0].level == 1
    assert headings[0].text == "AW-BIL-001 Billing Dispute Policy"
    assert clauses[0].text.startswith("1.1 ")


def test_wrapped_lines_stay_in_one_block() -> None:
    data = b"1.1 This clause is\nwrapped on two lines.\n1.2 Next clause."
    blocks = parse(data, "text/plain")
    assert [b.text for b in blocks] == [
        "1.1 This clause is wrapped on two lines.",
        "1.2 Next clause.",
    ]


def test_unsupported_content_type() -> None:
    with pytest.raises(UnsupportedContentType):
        parse(b"{}", "application/json")


def test_every_manifest_document_parses() -> None:
    manifest = json.loads((DATA_DIR / "manifest.json").read_text(encoding="utf-8"))
    for doc in manifest:
        data = (DATA_DIR / doc["path"]).read_bytes()
        assert parse(data, doc["content_type"]), doc["path"]
