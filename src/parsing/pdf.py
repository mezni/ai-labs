from io import BytesIO

from pypdf import PdfReader

from core.models import Block
from parsing._blocks import blocks_from_text


def parse_pdf(data: bytes) -> list[Block]:
    reader = PdfReader(BytesIO(data))
    blocks: list[Block] = []
    for number, page in enumerate(reader.pages, start=1):
        blocks.extend(blocks_from_text(page.extract_text() or "", page=number))
    return blocks
