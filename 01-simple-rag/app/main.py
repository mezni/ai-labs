import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from dotenv import load_dotenv
from pypdf import PdfReader

from src.knowledge.embeddings import embed
from src.knowledge.knowledge_base import KnowledgeBase
from src.utils.logger import get_logger

load_dotenv()

log = get_logger(__name__)

POLICIES_DIR = Path(os.getenv("POLICY_OUTPUT_DIR", "data/policies"))


def read_policy_files() -> list[dict[str, str]]:
    documents = []
    for pdf_path in sorted(POLICIES_DIR.glob("*.pdf")):
        reader = PdfReader(str(pdf_path))
        text = "\n".join(page.extract_text() or "" for page in reader.pages)
        documents.append(
            {
                "filename": pdf_path.name,
                "pages": len(reader.pages),
                "text": text,
            }
        )
    return documents


def main() -> None:
    log.info("Loading policies from %s", POLICIES_DIR)
    documents = read_policy_files()
    log.info("Read %d policy files", len(documents))

    kb = KnowledgeBase()
    kb.build(documents, embed_fn=embed)

    log.info("Total chunks embedded & stored: %d", kb.index.ntotal)
    log.info("Index saved to:                 %s", kb._index_path)
    log.info("Metadata saved to:              %s", kb._metadata_path)


if __name__ == "__main__":
    main()