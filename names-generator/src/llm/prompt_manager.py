from utils.config_loader import DEFAULT_PROMPTS_PATH, load_prompts


class PromptManager:
    def __init__(self, prompts_path=DEFAULT_PROMPTS_PATH):
        self.prompts = load_prompts(prompts_path)

    def get(self, name: str) -> str:
        if name not in self.prompts:
            raise KeyError(f"Prompt template '{name}' not found")
        return self.prompts[name]

    def build(self, name: str, **kwargs) -> str:
        template = self.get(name)
        return template.format(**kwargs)