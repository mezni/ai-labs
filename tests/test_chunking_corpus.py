from pathlib import Path

from corpus import chunk_document, load_manifest

DATA_DIR = Path(__file__).resolve().parent.parent / "data"


def test_roaming_v2_chunks_keep_lineage() -> None:
    manifest = load_manifest(DATA_DIR)
    entry = next(e for e in manifest if e.doc_id == "AW-ROA-002" and e.version == 2)
    chunks = chunk_document(DATA_DIR, entry)
    zone_b = next(c for c in chunks if "$8 per GB" in c.text)
    assert zone_b.clause == "2.2"
    assert zone_b.heading_path == ("AW-ROA-002 Roaming Policy (v2)", "2. Zones")
    assert zone_b.chunk_id.startswith("AW-ROA-002-v2-")
    assert "Zone A" not in zone_b.text


def test_all_documents_chunk_cleanly() -> None:
    chunk_ids: list[str] = []
    for entry in load_manifest(DATA_DIR):
        chunks = chunk_document(DATA_DIR, entry)
        assert chunks, entry.path
        for c in chunks:
            assert c.heading_path
            assert c.token_count <= 200
            chunk_ids.append(c.chunk_id)
    assert len(chunk_ids) == len(set(chunk_ids))
