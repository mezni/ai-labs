from core.models import Block
from parsing._blocks import blocks_from_text


def parse_text(data: bytes) -> list[Block]:
    return blocks_from_text(data.decode("utf-8"))
