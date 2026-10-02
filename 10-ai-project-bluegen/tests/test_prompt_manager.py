from prompt_manager import PromptManager


def test_prompt_manager_returns_system_prompt() -> None:
    manager = PromptManager()

    prompt = manager.get_system_prompt()

    assert prompt
    assert "AI architecture assistant" in prompt


def test_prompt_manager_builds_user_prompt() -> None:
    manager = PromptManager()

    prompt = manager.build_user_prompt(
        "Build an AI document classifier."
    )

    assert "Build an AI document classifier." in prompt


def test_prompt_manager_returns_version() -> None:
    manager = PromptManager()

    assert manager.get_version() == "v1"