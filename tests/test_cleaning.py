from cleaning import clean, normalize_text
from core.models import Block


def test_normalize_text_collapses_whitespace_and_ligatures() -> None:
    assert normalize_text("  Roaming \u00a0 \n fee\u00ad  \ufb01nal ") == "Roaming fee final"


def test_clean_drops_empty_blocks_and_page_numbers() -> None:
    blocks = [
        Block(kind="paragraph", text="  "),
        Block(kind="paragraph", text="Page 3 of 10"),
        Block(kind="paragraph", text="1.1 Keep  me."),
    ]
    assert [b.text for b in clean(blocks)] == ["1.1 Keep me."]


def test_clean_drops_text_repeated_across_pages() -> None:
    blocks = [
        Block(kind="paragraph", text="Confidential", page=1),
        Block(kind="paragraph", text="1.1 First.", page=1),
        Block(kind="paragraph", text="Confidential", page=2),
        Block(kind="paragraph", text="2.1 Second.", page=2),
    ]
    assert [b.text for b in clean(blocks)] == ["1.1 First.", "2.1 Second."]
