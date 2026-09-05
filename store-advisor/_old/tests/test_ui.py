from pathlib import Path

from streamlit.testing.v1 import AppTest

APP_PATH = Path(__file__).resolve().parent.parent / "app" / "ui.py"
CLOSED_PORT_URL = "http://127.0.0.1:9"


def _app(monkeypatch) -> AppTest:
    monkeypatch.setenv("API_BASE_URL", CLOSED_PORT_URL)
    return AppTest.from_file(str(APP_PATH))


def test_ui_renders(monkeypatch) -> None:
    at = _app(monkeypatch)
    at.run(timeout=60)
    assert not at.exception


def test_ui_ask_flow_without_backend(monkeypatch) -> None:
    at = _app(monkeypatch)
    at.run(timeout=60)
    at.chat_input[0].set_value("hello").run(timeout=60)
    assert not at.exception
    assert any("Cannot reach the API" in e.value for e in at.error)