from io import BytesIO

from reportlab.pdfgen import canvas

from cleaning import clean
from parsing import parse


def make_pdf(pages: list[list[str]]) -> bytes:
    buffer = BytesIO()
    pdf = canvas.Canvas(buffer)
    for lines in pages:
        y = 800
        for line in lines:
            pdf.drawString(72, y, line)
            y -= 20
        pdf.showPage()
    pdf.save()
    return buffer.getvalue()


def test_pdf_keeps_pages_and_cleaning_removes_headers_and_footers() -> None:
    data = make_pdf(
        [
            [
                "Aether Wireless - Confidential",
                "1.1 Customers may dispute a charge within 60 days.",
                "Page 1 of 2",
            ],
            [
                "Aether Wireless - Confidential",
                "2.1 Credits appear on the next invoice.",
                "Page 2 of 2",
            ],
        ]
    )
    blocks = parse(data, "application/pdf")
    assert {b.page for b in blocks} == {1, 2}

    cleaned = clean(blocks)
    assert [(b.page, b.text) for b in cleaned] == [
        (1, "1.1 Customers may dispute a charge within 60 days."),
        (2, "2.1 Credits appear on the next invoice."),
    ]
