import tiktoken


class TokenizerAnalyzer:
    def __init__(self, encoding_name: str = "cl100k_base"):
        self.encoding = tiktoken.get_encoding(encoding_name)

    def encode(self, text: str) -> list[int]:
        return self.encoding.encode(text)

    def decode(self, token_ids: list[int]) -> str:
        return self.encoding.decode(token_ids)

    def count_tokens(self, text: str) -> int:
        return len(self.encode(text))

    def analyze(self, text: str) -> dict:
        token_ids = self.encode(text)

        return {
            "text": text,
            "token_count": len(token_ids),
            "token_ids": token_ids,
            "character_count": len(text),
        }


class ContextBudget:
    def __init__(
        self,
        context_limit: int,
        reserved_output_tokens: int,
    ):
        if context_limit <= 0:
            raise ValueError("Context limit must be positive")

        if not 0 <= reserved_output_tokens < context_limit:
            raise ValueError("Invalid output reservation")

        self.context_limit = context_limit
        self.reserved_output_tokens = reserved_output_tokens

    def calculate_remaining(self, input_tokens: int) -> int:
        if input_tokens < 0:
            raise ValueError("Input tokens cannot be negative")

        return self.context_limit - self.reserved_output_tokens - input_tokens

    def can_fit(self, input_tokens: int) -> bool:
        return self.calculate_remaining(input_tokens) >= 0
