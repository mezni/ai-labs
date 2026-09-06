# 🚀 Aether Wireless - Policy Document Generator

Automatically generates styled, print-ready A4 PDF policy documents for **Aether Wireless**, a cloud-native 5G/6G carrier, using WeasyPrint and reusable HTML/CSS templates.

## 📦 Features
- **Multi-Policy Generation:** Renders a library of 15 corporate policy PDFs (AUP, Billing, Privacy, Device Protection, Port-In, Returns & Refunds, and more) from a single script.
- **Professional Formatting:** Consistent branded header banner, metadata block, callout boxes, tables, and signature blocks across every document.
- **Automatic Pagination & Footers:** Page numbers ("Page X of Y"), document identity, and brand footer via CSS `@page` rules.
- **Reusable Template Engine:** Data-driven `policies_data` list plus a shared `CSS_STYLE` stylesheet for easy adding/updating documents.
- **Deterministic Output:** All PDFs are written to a single output directory with predictable filenames.

## 🛠️ Tech Stack
**Language:** Python 3.13
**PDF Rendering:** WeasyPrint (HTML/CSS → PDF)
**Markup:** HTML5 + CSS3
**Tooling:** uv (package manager & virtual environments)

## 🚀 Getting Started

Follow these steps to set up the project locally.

### Prerequisites
- Python 3.13 or higher
- [uv](https://docs.astral.sh/uv/) package manager
- System libraries required by WeasyPrint (`libpango`, `libcairo`, `gdk-pixbuf`)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com
   ```
2. Create the virtual environment and install dependencies:
   ```bash
   uv sync
   ```
3. Generate all policy PDFs:
   ```bash
   uv run python scripts/generate_docs.py
   ```

The generated PDFs will be written to `data/policies/`.

## 📝 License
Distributed under the MIT License. See `LICENSE` for more information.
