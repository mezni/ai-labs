import re
import unicodedata
from collections import defaultdict

from core.models import Block

_SPACES = re.compile(r"\s+")
_PAGE_NUMBER = re.compile(r"^page \d+( of \d+)?$", re.IGNORECASE)


def normalize_text(text: str) -> str:
    """Unicode-normalize (ligatures, nbsp), drop soft hyphens, collapse whitespace."""
    text = text.replace("\u00ad", "")
    text = unicodedata.normalize("NFKC", text)
    return _SPACES.sub(" ", text).strip()


def clean(blocks: list[Block]) -> list[Block]:
    """Normalize text, drop empty blocks, page numbers, and text repeated across pages."""
    normalized = [block.model_copy(update={"text": normalize_text(block.text)}) for block in blocks]
    kept = [b for b in normalized if b.text and not _PAGE_NUMBER.match(b.text)]

    pages_by_text: dict[str, set[int]] = defaultdict(set)
    for block in kept:
        if block.page is not None:
            pages_by_text[block.text].add(block.page)
    repeated = {text for text, pages in pages_by_text.items() if len(pages) >= 2}

    return [b for b in kept if not (b.page is not None and b.text in repeated)]
