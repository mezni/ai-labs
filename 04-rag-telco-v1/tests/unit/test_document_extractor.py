from src.documents import DocumentExtractor, PDFPage


def test_extracts_section_heading():

    pages = [
        PDFPage(
            page_number=1,
            text="""
            1. Overview

            This policy describes international roaming.
            """,
        ),
    ]

    extractor = DocumentExtractor()

    result = extractor.extract(pages)

    assert len(result) == 1
    assert result[0].section == "1"
    assert result[0].section_title == "Overview"

def test_section_is_carried_to_following_pages():

    pages = [
        PDFPage(
            page_number=1,
            text="2. Roaming Charges\n\nCustomers may incur charges.",
        ),
        PDFPage(
            page_number=2,
            text="Additional charges may apply.",
        ),
    ]

    extractor = DocumentExtractor()

    result = extractor.extract(pages)

    assert result[0].section == "2"
    assert result[0].section_title == "Roaming Charges"

    assert result[1].section == "2"
    assert result[1].section_title == "Roaming Charges"