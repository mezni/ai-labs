import re

from core.models import Block, Chunk

CLAUSE = re.compile(r"^(\d+(?:\.\d+)+)\s")  # "2.1 Some clause"
SENTENCE_BREAK = re.compile(r"(?<=[.!?])\s+")
CHARS_PER_TOKEN = 4


def estimate_tokens(text: str) -> int:
    """Cheap token estimate (about 4 characters per token). Swap for a real tokenizer later."""
    return max(1, (len(text) + CHARS_PER_TOKEN - 1) // CHARS_PER_TOKEN)


def _clause_of(text: str) -> str | None:
    match = CLAUSE.match(text)
    return match.group(1) if match else None


def _hard_split(text: str, max_chars: int) -> list[str]:
    """Split on words, and slice words longer than max_chars."""
    if len(text) <= max_chars:
        return [text]
    parts: list[str] = []
    current = ""
    for word in text.split():
        while len(word) > max_chars:
            if current:
                parts.append(current)
                current = ""
            parts.append(word[:max_chars])
            word = word[max_chars:]
        candidate = f"{current} {word}".strip()
        if len(candidate) <= max_chars:
            current = candidate
        else:
            parts.append(current)
            current = word
    if current:
        parts.append(current)
    return parts


def split_text(text: str, max_tokens: int) -> list[str]:
    """Split oversized text on sentence boundaries, then on words as a last resort."""
    max_chars = max_tokens * CHARS_PER_TOKEN
    pieces: list[str] = []
    current = ""
    for sentence in SENTENCE_BREAK.split(text):
        for part in _hard_split(sentence, max_chars):
            candidate = f"{current} {part}".strip()
            if len(candidate) <= max_chars:
                current = candidate
            else:
                if current:
                    pieces.append(current)
                current = part
    if current:
        pieces.append(current)
    return pieces


def _can_join(
    current_tokens: int, block_tokens: int, starts_clause: bool, min_tokens: int, max_tokens: int
) -> bool:
    if current_tokens + block_tokens > max_tokens:
        return False
    return current_tokens < min_tokens or not starts_clause


def chunk_blocks(
    blocks: list[Block],
    *,
    doc_id: str,
    version: int,
    domain: str,
    max_tokens: int = 200,
    min_tokens: int = 20,
) -> list[Chunk]:
    """Group blocks into chunks that respect headings and clause boundaries."""
    chunks: list[Chunk] = []
    stack: list[tuple[int, str]] = []  # (heading level, heading text)
    pending: list[Block] = []
    pending_tokens = 0

    def heading_path() -> tuple[str, ...]:
        return tuple(text for _, text in stack)

    def emit(text: str, clause: str | None, page: int | None) -> None:
        ordinal = len(chunks)
        chunks.append(
            Chunk(
                chunk_id=f"{doc_id}-v{version}-{ordinal:04d}",
                doc_id=doc_id,
                version=version,
                domain=domain,
                ordinal=ordinal,
                text=text,
                heading_path=heading_path(),
                clause=clause,
                page=page,
                token_count=estimate_tokens(text),
            )
        )

    def flush() -> None:
        nonlocal pending_tokens
        if pending:
            text = " ".join(block.text for block in pending)
            emit(text, _clause_of(pending[0].text), pending[0].page)
            pending.clear()
            pending_tokens = 0

    for block in blocks:
        if block.kind == "heading":
            flush()
            level = block.level or 1
            stack = [(lvl, text) for lvl, text in stack if lvl < level] + [(level, block.text)]
            continue

        tokens = estimate_tokens(block.text)
        if tokens > max_tokens:
            flush()
            clause = _clause_of(block.text)
            for piece in split_text(block.text, max_tokens):
                emit(piece, clause, block.page)
            continue

        starts_clause = _clause_of(block.text) is not None
        if pending and not _can_join(pending_tokens, tokens, starts_clause, min_tokens, max_tokens):
            flush()
        pending.append(block)
        pending_tokens += tokens

    flush()
    return chunks
