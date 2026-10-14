import re

from core.models import Block

_HEADING = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$")
_CLAUSE = re.compile(r"^\d+(?:\.\d+)+\s")
_SENTENCE_END = re.compile(r"[.!?]$")


def blocks_from_text(
    text: str,
    *,
    detect_headings: bool = False,
    page: int | None = None,
) -> list[Block]:
    """Split plain or lightly-marked text into heading and paragraph blocks.

    Numbered clauses start a new block and wrapped lines are joined onto the
    clause they belong to. A line also starts a new block when the previous
    line ends a sentence, which keeps trailing headers, footers and page
    numbers as separate blocks for the cleaning stage to drop.
    """
    blocks: list[Block] = []
    current: list[str] | None = None

    def flush() -> None:
        if current is not None:
            blocks.append(Block(kind="paragraph", text=" ".join(current), page=page))

    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue

        if detect_headings and (heading := _HEADING.match(line)):
            flush()
            current = None
            blocks.append(
                Block(
                    kind="heading",
                    text=heading.group(2).strip(),
                    level=len(heading.group(1)),
                    page=page,
                )
            )
            continue

        if current is None or _CLAUSE.match(line) or _SENTENCE_END.search(current[-1]):
            flush()
            current = [line]
        else:
            current.append(line)

    flush()
    return blocks
