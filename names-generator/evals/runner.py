import json
from pathlib import Path

from dotenv import load_dotenv

from agents.name_generator_agent import NameGeneratorAgent
from evals.evaluators import diversity, format, relevance

load_dotenv()

DATASET_PATH = Path(__file__).parent / "datasets" / "name_generator.json"
EVALUATORS = [format.evaluate, relevance.evaluate, diversity.evaluate]


def _print_result(item, result, index):
    status = "PASS" if result["passed"] else "FAIL"
    print(f"    [{status}] {result['evaluator'].upper():10s} score={result['score']:.2f}")
    if isinstance(result["detail"], dict):
        extra = result["detail"].get("count_ok")
        if extra is not None:
            print(f"           {result['detail']}")


def main():
    dataset = json.loads(DATASET_PATH.open())
    agent = NameGeneratorAgent()
    expected = agent.expected

    print(f"Name Generator evals — {len(dataset)} cases, expected {expected} names")
    print()

    totals = {name: {"sum": 0.0, "n": 0, "passed": 0} for name in ["format", "relevance", "diversity"]}

    for item in dataset:
        idea = item["input"]
        print(f"Case: {idea}")
        print(f"  expected_characteristics: {', '.join(item['expected_characteristics'])}")

        try:
            candidates = agent.generate(idea)
        except Exception as error:
            print(f"  [ERROR] generation failed: {error}")
            print()
            continue

        for evaluator in EVALUATORS:
            result = evaluator(candidates, idea, expected_count=expected)
            _print_result(item, result, 0)
            totals[result["evaluator"]]["sum"] += result["score"]
            totals[result["evaluator"]]["n"] += 1
            totals[result["evaluator"]]["passed"] += int(result["passed"])
        print()

    print("--- Aggregate ---")
    for name, stats in totals.items():
        if stats["n"] == 0:
            print(f"  {name:10s}: no runs")
            continue
        avg = stats["sum"] / stats["n"]
        print(
            f"  {name:10s}: avg={avg:.2f}  passed={stats['passed']}/{stats['n']}"
        )


if __name__ == "__main__":
    main()