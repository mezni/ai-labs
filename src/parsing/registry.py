from collections.abc import Callable

from core.models import Block
from parsing.markdown import parse_markdown
from parsing.pdf import parse_pdf
from parsing.text import parse_text

Parser = Callable[[bytes], list[Block]]

PARSERS: dict[str, Parser] = {
    "text/markdown": parse_markdown,
    "text/plain": parse_text,
    "application/pdf": parse_pdf,
}


class UnsupportedContentType(ValueError):
    """Raised when no parser is registered for a content type."""


def parse(data: bytes, content_type: str) -> list[Block]:
    try:
        parser = PARSERS[content_type]
    except KeyError:
        raise UnsupportedContentType(content_type) from None
    return parser(data)
