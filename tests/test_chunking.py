from chunking import chunk_blocks, estimate_tokens
from core.models import Block


def h(text: str, level: int) -> Block:
    return Block(kind="heading", text=text, level=level)


def p(text: str) -> Block:
    return Block(kind="paragraph", text=text)


def chunk(blocks: list[Block], **kwargs):
    return chunk_blocks(blocks, doc_id="D", version=1, domain="test", **kwargs)


def test_estimate_tokens() -> None:
    assert estimate_tokens("") == 1
    assert estimate_tokens("abcd") == 1
    assert estimate_tokens("abcde") == 2


def test_heading_path_clause_and_ids() -> None:
    blocks = [
        h("Policy", 1),
        h("1. Scope", 2),
        p("1.1 First clause."),
        h("2. Zones", 2),
        p("2.1 Second clause."),
        p("2.2 Third clause."),
    ]
    chunks = chunk(blocks, max_tokens=100, min_tokens=1)
    assert [c.heading_path for c in chunks] == [
        ("Policy", "1. Scope"),
        ("Policy", "2. Zones"),
        ("Policy", "2. Zones"),
    ]
    assert [c.clause for c in chunks] == ["1.1", "2.1", "2.2"]
    assert [c.chunk_id for c in chunks] == ["D-v1-0000", "D-v1-0001", "D-v1-0002"]
    assert [c.ordinal for c in chunks] == [0, 1, 2]


def test_small_clauses_are_merged() -> None:
    blocks = [h("Policy", 1), p("1.1 Short."), p("1.2 Also short.")]
    chunks = chunk(blocks, max_tokens=100, min_tokens=20)
    assert len(chunks) == 1
    assert chunks[0].text == "1.1 Short. 1.2 Also short."
    assert chunks[0].clause == "1.1"


def test_oversized_block_is_split_on_sentences() -> None:
    text = " ".join(f"This is sentence number {i} of the test." for i in range(10))
    chunks = chunk([h("Policy", 1), p(text)], max_tokens=20)
    assert len(chunks) == 5
    assert all(c.token_count <= 20 for c in chunks)
    assert all(c.heading_path == ("Policy",) for c in chunks)
    assert " ".join(c.text for c in chunks) == text


def test_oversized_sentence_falls_back_to_words() -> None:
    text = " ".join(["word"] * 30)
    chunks = chunk([h("Policy", 1), p(text)], max_tokens=10)
    assert len(chunks) == 4
    assert all(c.token_count <= 10 for c in chunks)


def test_text_with_context_prefixes_heading_path() -> None:
    chunks = chunk([h("Policy", 1), h("1. Scope", 2), p("1.1 First clause.")])
    assert chunks[0].text_with_context() == "Policy > 1. Scope\n1.1 First clause."
