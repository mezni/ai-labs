import json
from collections import Counter
from datetime import date
from pathlib import Path

DATA_DIR = Path(__file__).resolve().parent.parent / "data"
MANIFEST = json.loads((DATA_DIR / "manifest.json").read_text(encoding="utf-8"))


def test_manifest_files_exist() -> None:
    for doc in MANIFEST:
        assert (DATA_DIR / doc["path"]).is_file(), f"missing file: {doc['path']}"


def test_doc_id_and_version_are_unique() -> None:
    keys = [(d["doc_id"], d["version"]) for d in MANIFEST]
    assert len(keys) == len(set(keys))


def test_dates_are_valid_and_ordered() -> None:
    for doc in MANIFEST:
        start = date.fromisoformat(doc["effective_from"])
        if doc["effective_to"] is not None:
            assert date.fromisoformat(doc["effective_to"]) > start


def test_one_current_version_per_doc() -> None:
    current = Counter(d["doc_id"] for d in MANIFEST if d["status"] == "current")
    assert all(count == 1 for count in current.values())
    assert set(current) == {d["doc_id"] for d in MANIFEST}
