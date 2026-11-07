from dataclasses import dataclass


@dataclass
class InferenceMetrics:
    request_start: float
    first_token_time: float
    generation_end: float
    generated_tokens: int

    def validate(self):
        if self.request_start > self.first_token_time:
            raise ValueError("First token cannot precede request")

        if self.first_token_time > self.generation_end:
            raise ValueError("Generation cannot end before first token")

        if self.generated_tokens < 1:
            raise ValueError("At least one token is required")

    @property
    def ttft(self) -> float:
        self.validate()
        return self.first_token_time - self.request_start

    @property
    def total_latency(self) -> float:
        self.validate()
        return self.generation_end - self.request_start

    @property
    def generation_duration(self) -> float:
        self.validate()
        return self.generation_end - self.first_token_time

    @property
    def tokens_per_second(self) -> float:
        duration = self.generation_duration

        if duration == 0:
            return 0.0

        return self.generated_tokens / duration

    def report(self) -> dict:
        return {
            "ttft_seconds": round(self.ttft, 4),
            "total_latency_seconds": round(self.total_latency, 4),
            "generation_duration_seconds": round(self.generation_duration, 4),
            "generated_tokens": self.generated_tokens,
            "tokens_per_second": round(self.tokens_per_second, 2),
        }
