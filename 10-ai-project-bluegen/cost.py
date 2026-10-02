from dataclasses import dataclass


@dataclass(frozen=True)
class ModelPricing:
    input_cost_per_million_tokens: float
    output_cost_per_million_tokens: float


@dataclass(frozen=True)
class GenerationCost:
    input_cost: float
    output_cost: float
    total_cost: float


class CostCalculator:

    def calculate(
        self,
        pricing: ModelPricing,
        input_tokens: int,
        output_tokens: int,
    ) -> GenerationCost:

        input_cost = (
            input_tokens / 1_000_000
        ) * pricing.input_cost_per_million_tokens

        output_cost = (
            output_tokens / 1_000_000
        ) * pricing.output_cost_per_million_tokens

        return GenerationCost(
            input_cost=input_cost,
            output_cost=output_cost,
            total_cost=input_cost + output_cost,
        )