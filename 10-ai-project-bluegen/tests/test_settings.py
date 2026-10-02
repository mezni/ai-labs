from config import Settings


def test_settings_reads_openrouter_api_key(monkeypatch) -> None:
    monkeypatch.setenv(
        "OPENROUTER_API_KEY",
        "test-key",
    )

    settings = Settings()

    assert settings.openrouter_api_key == "test-key"