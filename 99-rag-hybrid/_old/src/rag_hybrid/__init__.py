"""Console entry point: launch the Streamlit Q&A app."""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

_APP_PATH = (
    Path(__file__).resolve().parent.parent.parent / "app" / "streamlit_app.py"
)


def main() -> int:
    """Run the Streamlit app in a subprocess."""
    return subprocess.call([sys.executable, "-m", "streamlit", "run", str(_APP_PATH)])


if __name__ == "__main__":
    raise SystemExit(main())