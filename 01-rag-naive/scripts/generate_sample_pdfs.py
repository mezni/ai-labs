"""Generate sample policy PDFs into data/raw/ for development.

Builds minimal, valid PDFs (catalog/pages/page/content/font objects with a
correct xref table) so the extraction layer has real multi-page input.
"""

from __future__ import annotations

import pathlib

OUT_DIR = pathlib.Path(__file__).resolve().parent.parent / "data" / "raw"

BILLING_POLICY = [
    """Aurora Mobile Billing Policy
This document describes the billing and refund policy for Aurora Mobile customers. It replaces all previously published billing documents and applies to all current and future accounts.""",
    """Refund Policy
A customer may request a refund within 30 days of the original billing date. Refund requests are accepted for unused services only. To request a refund, the customer must contact support through the mobile application and provide the account identifier and the original invoice number.""",
    """Late Payments
Payments are due on the first day of each billing cycle. If payment is not received within 10 days of the due date, a late fee of 5 percent of the outstanding balance is applied. Accounts dormant for more than 60 days may be suspended until the balance is settled.""",
    """Disputes and Exceptions
Customers may dispute a charge within 90 days of the statement date. Disputes are reviewed within 15 business days. Exceptions to the standard refund window may apply to enterprise accounts with a valid service-level agreement on file.""",
]

ROAMING_POLICY = [
    """Aurora Mobile Roaming Policy
This policy explains how roaming services are billed when customers travel outside the home network. Roaming is available in three zones: zone one, zone two, and zone three.""",
    """Zone Rates
Zone one includes neighboring countries and is billed at the standard rate. Zone two includes the rest of the region and is billed at twice the standard rate. Zone three includes all other countries and is billed at three times the standard rate.""",
    """Data and Usage Limits
Roaming data is limited to 2 gigabytes per month at the zone one rate. Daily roaming notifications are sent when usage reaches 80 percent of the included allowance. Customers may enable travel passes to reduce per-megabyte costs during international travel.""",
]


def escape(text: str) -> str:
    return text.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)")


def wrap(text: str, width: int = 88) -> list[str]:
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


def content_stream(text: str) -> bytes:
    stream_lines = []
    y = 738
    for line in wrap(text):
        if y < 40:
            break
        stream_lines.append(f"BT /F1 11 Tf 72 {y} Td ({escape(line)}) Tj ET")
        y -= 15
    stream = "\n".join(stream_lines).encode("latin-1")
    length = str(len(stream)).encode("ascii")
    return b"<< /Length " + length + b" >>\nstream\n" + stream + b"\nendstream"


def build_pdf(pages: list[str]) -> bytes:
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
        objects.append(content_stream(text))

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


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    documents = {
        "billing_policy.pdf": BILLING_POLICY,
        "roaming_policy.pdf": ROAMING_POLICY,
    }
    for name, pages in documents.items():
        target = OUT_DIR / name
        target.write_bytes(build_pdf(pages))
        print(f"wrote {target}")


if __name__ == "__main__":
    main()
