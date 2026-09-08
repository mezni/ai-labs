from uuid import uuid4

from src.documents import ExtractedPage
from src.indexing import ChunkingService


def test_text_is_split_into_chunks():

    pages = [
        ExtractedPage(
            page_number=1,
            text="A" * 1000,
            section="1",
            section_title="Overview",
        )
    ]

    document_id = uuid4()
    version_id = uuid4()

    chunker = ChunkingService(
        chunk_size=500,
        chunk_overlap=50,
    )

    chunks = chunker.chunk_pages(
        pages,
        document_id=document_id,
        version_id=version_id,
    )

    assert len(chunks) == 3

    assert len(chunks[0].chunk_text) == 500
    assert len(chunks[1].chunk_text) == 500
    assert len(chunks[2].chunk_text) == 100


def test_chunk_preserves_page_and_section_metadata():

    pages = [
        ExtractedPage(
            page_number=7,
            text="Customers travelling internationally may incur roaming charges.",
            section="2",
            section_title="Roaming Charges",
        )
    ]

    chunker = ChunkingService()

    chunks = chunker.chunk_pages(
        pages,
        document_id=uuid4(),
        version_id=uuid4(),
    )

    assert len(chunks) == 1

    chunk = chunks[0]

    assert chunk.page_number == 7
    assert chunk.section == "2"
    assert chunk.section_title == "Roaming Charges"

    assert chunk.metadata["page_number"] == "7"
    assert chunk.metadata["section"] == "2"

def test_chunks_have_overlap():

    text = "0123456789" * 100

    pages = [
        ExtractedPage(
            page_number=1,
            text=text,
        )
    ]

    chunker = ChunkingService(
        chunk_size=100,
        chunk_overlap=20,
    )

    chunks = chunker.chunk_pages(
        pages,
        document_id=uuid4(),
        version_id=uuid4(),
    )

    assert chunks[0].chunk_text[-20:] == chunks[1].chunk_text[:20]