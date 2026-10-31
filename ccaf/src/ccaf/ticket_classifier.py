from ccaf.llm import ClaudeClient


class TicketClassifier:
    def __init__(self, client: ClaudeClient):
        self._client = client

    def classify(self, message: str):
        prompt = f"Classify this ticket  {message}"
        return self._client.generate(prompt)
