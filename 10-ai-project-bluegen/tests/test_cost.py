from cost import CostCalculator, ModelPricing


def test_calculates_generation_cost():
    calculator = CostCalculator()

    pricing = ModelPricing(
        input_cost_per_million_tokens=1.0,
        output_cost_per_million_tokens=2.0,
    )

    result = calculator.calculate(
        pricing=pricing,
        input_tokens=100_000,
        output_tokens=50_000,
    )

    assert result.input_cost == 0.1
    assert result.output_cost == 0.1
    assert result.total_cost == 0.2


def test_zero_tokens_have_zero_cost():
    calculator = CostCalculator()

    pricing = ModelPricing(
        input_cost_per_million_tokens=1.0,
        output_cost_per_million_tokens=2.0,
    )

    result = calculator.calculate(
        pricing=pricing,
        input_tokens=0,
        output_tokens=0,
    )

    assert result.total_cost == 0.0