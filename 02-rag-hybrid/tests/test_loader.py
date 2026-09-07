"""Tests for KnowledgeBase.load_documents."""


def _write(dir_path, relative_path: str, text: str) -> None:
    path = dir_path / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def test_load_documents_reads_markdown_recursively(tmp_path, kb_factory):
    _write(tmp_path, "hr/leave.md", "Leave policy allows thirty days per year.")
    _write(tmp_path, "it/access.md", "Access requires manager approval.")
    _write(tmp_path, "notes/backup.md", "Backups run nightly.")
    _write(tmp_path, "ignored.txt", "not markdown")

    records = kb_factory(corpus_dir=tmp_path).load_documents()

    assert [record["source"] for record in records] == [
        "hr/leave.md",
        "it/access.md",
        "notes/backup.md",
    ]
    for record in records:
        assert set(record) == {
            "chunk_id",
            "content",
            "document_id",
            "source",
            "category",
        }


def test_load_documents_derives_categories_and_document_ids(tmp_path, kb_factory):
    _write(tmp_path, "hr/leave.md", "Some leave content.")
    _write(tmp_path, "it/access.md", "Some access content.")

    records = kb_factory(corpus_dir=tmp_path).load_documents()

    assert {record["category"] for record in records} == {"hr", "it"}
    assert {record["document_id"] for record in records} == {"leave", "access"}


def test_load_documents_bare_root_document_uses_corpus_name(tmp_path, kb_factory):
    _write(tmp_path, "overview.md", "Overview content.")

    records = kb_factory(corpus_dir=tmp_path).load_documents()

    assert records[0]["category"] == tmp_path.name


def test_load_documents_skips_empty_files(tmp_path, kb_factory):
    _write(tmp_path, "hr/leave.md", "   \n  ")
    _write(tmp_path, "it/access.md", "Access content.")

    records = kb_factory(corpus_dir=tmp_path).load_documents()

    assert [record["source"] for record in records] == ["it/access.md"]