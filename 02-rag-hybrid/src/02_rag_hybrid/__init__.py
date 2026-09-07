"""02-rag-hybrid package entry point."""


def main() -> None:
    """Launch the Streamlit Q&A app."""
    import subprocess
    import sys
    from pathlib import Path

    app_path = Path(__file__).resolve().parent.parent.parent / "app" / "streamlit_app.py"
    raise SystemExit(
        subprocess.call([sys.executable, "-m", "streamlit", "run", str(app_path)])
    )