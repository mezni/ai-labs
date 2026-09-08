"""Minimal PDF builder for tests.

Produces valid single/multi-page PDFs (catalog, pages, page, content, and font
objects with a correct xref table) so extraction can be tested without shipping
binary fixtures.
"""


def _escape(text: str) -> str:
    return text.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")


def _wrap(text: str, width: int = 88) -> list[str]:
    lines: list[str] = []
    for raw_line in text.splitlines():
        words = raw_line.split()
        current = ""
        for word in words:
            candidate = f"{current} {word}" if current else word
            if len(candidate) <= width:
                current = candidate
            else:
                lines.append(current)
                current = word
        if current:
            lines.append(current)
    return lines


def _content_stream(text: str) -> bytes:
    stream_lines = []
    y = 738
    for line in _wrap(text):
        if y < 40:
            break
        stream_lines.append(f"BT /F1 11 Tf 72 {y} Td ({_escape(line)}) Tj ET")
        y -= 15
    stream = "\n".join(stream_lines).encode("latin-1")
    return b"<< /Length " + str(len(stream)).encode() + b" >>\nstream\n" + stream + b"\nendstream"


def build_pdf(pages: list[str]) -> bytes:
    """Build a valid PDF with one page per string in ``pages``."""
    objects: list[bytes] = [b""]
    font_number = 2 + 2 * len(pages) + 1

    objects.append(b"<< /Type /Catalog /Pages 2 0 R >>")

    kids = " ".join(f"{3 + 2 * i} 0 R" for i in range(len(pages)))
    objects.append(f"<< /Type /Pages /Kids [{kids}] /Count {len(pages)} >>".encode("ascii"))

    for index, text in enumerate(pages):
        page_number = 3 + 2 * index
        content_number = page_number + 1
        objects.append(
            (
                f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] "
                f"/Contents {content_number} 0 R "
                f"/Resources << /Font << /F1 {font_number} 0 R >> >> >>"
            ).encode("ascii")
        )
        objects.append(_content_stream(text))

    objects.append(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")

    body = bytearray(b"%PDF-1.4\n")
    offsets: list[int] = []
    for number in range(1, len(objects)):
        offsets.append(len(body))
        body += f"{number} 0 obj\n".encode("ascii")
        body += objects[number]
        body += b"\nendobj\n"

    xref_position = len(body)
    object_count = len(objects) - 1
    body += f"xref\n0 {object_count + 1}\n".encode("ascii")
    body += b"0000000000 65535 f \n"
    for offset in offsets:
        body += f"{offset:010d} 00000 n \n".encode("ascii")
    body += (
        f"trailer\n<< /Size {object_count + 1} /Root 1 0 R >>\nstartxref\n{xref_position}\n%%EOF\n"
    ).encode("ascii")
    return bytes(body)
