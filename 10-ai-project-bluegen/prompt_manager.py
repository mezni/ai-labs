from pathlib import Path

import yaml

from interfaces import PromptManagerInterface


PROMPTS_CONFIG_PATH = Path("config/prompts.yaml")


class PromptManager(PromptManagerInterface):

    def __init__(
        self,
        config_path: Path = PROMPTS_CONFIG_PATH,
    ) -> None:

        self.config_path = config_path
        self.config = self._load_config()

    def _load_config(self) -> dict:

        if not self.config_path.exists():
            raise FileNotFoundError(
                f"Prompt configuration not found: "
                f"{self.config_path}"
            )

        with self.config_path.open(
            "r",
            encoding="utf-8",
        ) as file:
            config = yaml.safe_load(file)

        if not config:
            raise ValueError(
                "Prompt configuration is empty."
            )

        return config

    def get_system_prompt(self) -> str:

        project_config = self.config[
            "project_blueprint"
        ]

        version = project_config["version"]
        base_path = Path(
            project_config["path"]
        )

        prompt_path = (
            base_path
            / version
            / "system.txt"
        )

        return self._load_prompt(prompt_path)

    def build_user_prompt(
        self,
        project_idea: str,
    ) -> str:

        project_config = self.config[
            "project_blueprint"
        ]

        version = project_config["version"]
        base_path = Path(
            project_config["path"]
        )

        prompt_path = (
            base_path
            / version
            / "user.txt"
        )

        template = self._load_prompt(
            prompt_path
        )

        return template.format(
            project_idea=project_idea,
        )

    def get_version(self) -> str:
        return self.config[
            "project_blueprint"
        ]["version"]

    def _load_prompt(
        self,
        path: Path,
    ) -> str:

        if not path.exists():
            raise FileNotFoundError(
                f"Prompt file not found: {path}"
            )

        content = path.read_text(
            encoding="utf-8"
        ).strip()

        if not content:
            raise ValueError(
                f"Prompt file is empty: {path}"
            )

        return content