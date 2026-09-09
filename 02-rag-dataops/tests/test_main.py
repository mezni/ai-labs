from __future__ import annotations

import src.main as cli
from src.indexing.registry import IndexRegistry, IndexVersionConfig
from src.ingestion.hashing import DocumentCatalog
from src.ingestion.loader import Loader
from src.main import _parse_args, run_catalog, run_index, run_rollback, run_versions
from tests.factories import build_pdf, build_txt


def test_cli_dispatch_commands():
    args = _parse_args(["index"])
    assert args.func is run_index
    args = _parse_args(["versions"])
    assert args.func is run_versions
    args = _parse_args(["catalog"])
    assert args.func is run_catalog
    assert _parse_args(["rollback", "v1"]).version == "v1"
    assert _parse_args(["search", "question"]).query == "question"


def test_index_builds_first_snapshot(settings, monkeypatch, fake_embedder):
    build_pdf(settings.data_dir / "a.pdf", "First doc")
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)

    assert run_index(settings) == 0
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.current_version == "v1"
    assert registry.versions["v1"].chunk_count == 1
    assert registry.versions["v1"].document_count == 1

    catalog = DocumentCatalog().load(settings.document_catalog)
    record = catalog.get("a.pdf")
    assert record.version == "v1"
    assert record.chunk_ids


def test_index_unmatched_unchanged_is_idempotent(indexed, monkeypatch, fake_embedder):
    calls = []

    class CountingEmbedder(fake_embedder.__class__):
        def embed(self, texts):
            calls.append(list(texts))
            return super().embed(texts)

    counting = CountingEmbedder(
        api_key="k",
        base_url=fake_embedder.base_url,
        model=fake_embedder.model,
        client=fake_embedder._client,
    )
    monkeypatch.setattr(cli, "build_embedder", lambda s: counting)

    assert run_index(indexed) == 0
    registry = IndexRegistry().load(indexed.index_registry)
    assert registry.current_version == "v1"  # identical run → no new snapshot
    assert set(registry.versions) == {"v1"}
    assert calls == []  # no embedding requests on a no-op run


def test_index_changed_document_bumps_snapshot(indexed, monkeypatch, fake_embedder):
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)
    build_pdf(indexed.data_dir / "a.pdf", "Refund policy text for testing V2 CHANGED")

    assert run_index(indexed) == 0
    registry = IndexRegistry().load(indexed.index_registry)
    assert set(registry.versions) == {"v1", "v2"}
    assert registry.current_version == "v2"
    catalog = DocumentCatalog().load(indexed.document_catalog)
    record = catalog.get("a.pdf")
    assert record.version == "v2"
    assert record.history[-1].version == "v1"


def test_index_deleted_document_removed(indexed, monkeypatch, fake_embedder):
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)
    (indexed.data_dir / "a.pdf").unlink()

    assert run_index(indexed) == 0
    catalog = DocumentCatalog().load(indexed.document_catalog)
    assert catalog.get("a.pdf") is None
    registry = IndexRegistry().load(indexed.index_registry)
    assert registry.current_version == "v2"
    assert registry.versions["v2"].chunk_count == 0


def test_index_parses_multiple_source_types(settings, monkeypatch, fake_embedder):
    build_pdf(settings.data_dir / "policy.pdf", "policy pdf")
    build_txt(settings.data_dir / "notes.txt", "plain notes")
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)

    assert run_index(settings) == 0
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.current_version == "v1"
    assert registry.versions["v1"].document_count == 2
    assert registry.versions["v1"].stats == {"pdf": 1, "txt": 1}


def test_per_document_parse_failure_continues(settings, monkeypatch, fake_embedder):
    build_pdf(settings.data_dir / "good.pdf", "good")
    broken = settings.data_dir / "broken.pdf"
    broken.write_bytes(b"definitely not a pdf")
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)

    assert run_index(settings) == 0  # failure is logged + skipped, not fatal
    registry = IndexRegistry().load(settings.index_registry)
    assert registry.versions["v1"].document_count == 1


def test_embedding_model_change_triggers_full_reindex(
    indexed, monkeypatch, fake_embedder
):
    other = fake_embedder.__class__(
        api_key="k",
        base_url=fake_embedder.base_url,
        model="different-model",
        client=fake_embedder._client,
    )
    monkeypatch.setattr(cli, "build_embedder", lambda s: other)
    indexed.openrouter_embedding_model = "different-model"

    assert run_index(indexed) == 0
    registry = IndexRegistry().load(indexed.index_registry)
    assert set(registry.versions) == {"v1", "v2"}
    assert registry.current_version == "v2"


def test_versions_and_catalog_commands(settings):
    registry = IndexRegistry()
    registry.register(
        "v1",
        IndexVersionConfig(
            collection_name="docs_v1", embedding_model="test-embed", chunk_count=3
        ),
    )
    registry.set_current("v1")
    registry.save(settings.index_registry)

    assert run_versions(settings) == 0
    assert run_catalog(settings) == 0


def test_rollback_via_cli(indexed, monkeypatch, fake_embedder):
    monkeypatch.setattr(cli, "build_embedder", lambda s: fake_embedder)
    build_pdf(indexed.data_dir / "a.pdf", "changed content V2")
    run_index(indexed)
    assert IndexRegistry().load(indexed.index_registry).current_version == "v2"
    assert run_rollback(indexed, "v1") == 0
    assert IndexRegistry().load(indexed.index_registry).current_version == "v1"


def test_discovery_uses_loader(settings):
    build_txt(settings.data_dir / "x.txt", "x")
    found = Loader(settings).discover()
    assert any(d.document_id == "x.txt" for d in found)