from __future__ import annotations

from src.ingestion.loader import (
    CONFLUENCE_PROFILE,
    DATABASE_PROFILE,
    FILESYSTEM_PROFILE,
    SHAREPOINT_PROFILE,
    SOURCE_PROFILES,
    WEB_PROFILE,
    Loader,
)
from tests.factories import build_pdf, build_sample_database, build_txt


def test_all_five_source_profiles_registered():
    assert set(SOURCE_PROFILES) == {
        "filesystem",
        "web",
        "confluence",
        "sharepoint",
        "database",
    }
    assert FILESYSTEM_PROFILE.connector == "filesystem"
    assert WEB_PROFILE.connector == "web"
    assert CONFLUENCE_PROFILE.connector == "confluence"
    assert SHAREPOINT_PROFILE.connector == "sharepoint"
    assert DATABASE_PROFILE.connector == "database"


def test_recursive_filesystem_discovery(settings):
    build_pdf(settings.data_dir / "top.pdf", "top level")
    build_pdf(settings.data_dir / "nested" / "inner.pdf", "nested")
    build_txt(settings.data_dir / "nested" / "deep" / "notes.txt", "deep")

    loader = Loader(settings)
    documents = loader.discover("filesystem")
    ids = {d.document_id for d in documents}
    assert "top.pdf" in ids
    assert "nested/inner.pdf" in ids
    assert "nested/deep/notes.txt" in ids
    assert ids == {"top.pdf", "nested/inner.pdf", "nested/deep/notes.txt"}
    top = next(d for d in documents if d.document_id == "top.pdf")
    assert top.source == "filesystem"
    assert top.format == "pdf"


def test_database_connector_discovery(settings):
    db = settings.data_dir / "sample.db"
    build_sample_database(db, [(1, "A", "body a"), (2, "B", "body b")])

    documents = Loader(settings).discover("database")
    assert [d.document_id for d in documents] == ["documents/1", "documents/2"]
    assert documents[0].source == "database"
    assert documents[0].format == "database_row"
    assert documents[0].metadata["table"] == "documents"
    assert documents[0].metadata["row"]["title"] == "A"


def test_database_connector_missing_file_is_empty(settings):
    assert Loader(settings).discover("database") == []


def test_loader_routes_to_parser(settings):
    build_pdf(settings.data_dir / "p.pdf", "parsed content")
    loader = Loader(settings)
    discovered = loader.discover("filesystem")[0]
    parsed = loader.load(discovered)
    assert parsed.document_id == "p.pdf"
    assert parsed.format == "pdf"
    assert "parsed content" in parsed.text


def test_database_row_route(settings):
    db = settings.data_dir / "sample.db"
    build_sample_database(db, [(1, "Title", "Body")])
    loader = Loader(settings)
    discovered = loader.discover("database")[0]
    parsed = loader.load(discovered)
    assert parsed.source == "database"
    assert parsed.format == "database_row"
    assert "Title" in parsed.text
    assert parsed.metadata["primary_key"] == "id"


def test_discover_defaults_filesystem_and_database(settings):
    build_pdf(settings.data_dir / "p.pdf", "x")
    build_sample_database(settings.data_dir / "sample.db", [(1, "A", "b")])
    loader = Loader(settings)
    ids = {d.document_id for d in loader.discover()}
    assert "p.pdf" in ids
    assert "documents/1" in ids


def test_source_metadata_passthrough(settings):
    build_pdf(settings.data_dir / "m.pdf", "meta")
    loader = Loader(settings)
    discovered = loader.discover("filesystem")[0]
    assert "path" in discovered.metadata
    parsed = loader.load(discovered)
    assert "path" in parsed.metadata