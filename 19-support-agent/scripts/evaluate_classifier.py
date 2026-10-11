import json
import sys
from pathlib import Path

from support_agent.llm_classifier import classify_ticket

DATASET_PATH = Path("datasets/ticket_eval.jsonl")


def load_dataset():
    cases = []

    with DATASET_PATH.open(encoding="utf-8") as file:
        for line_number, line in enumerate(file, start=1):
            if not line.strip():
                continue

            try:
                cases.append(json.loads(line))
            except json.JSONDecodeError as exc:
                raise ValueError(f"Invalid JSON on line {line_number}") from exc

    return cases


def main():
    cases = load_dataset()
    if not cases:
        raise ValueError("Evaluation dataset is empty.")

    category_correct = 0
    exact_correct = 0
    failures = 0

    for index, case in enumerate(cases, start=1):
        ticket = case["ticket"]
        expected = case["expected"]

        print(f"\n[{index}/{len(cases)}] {ticket}")

        try:
            actual = classify_ticket(ticket).model_dump()
        except Exception as exc:
            failures += 1
            print(f"ERROR: {type(exc).__name__}: {exc}")
            continue

        category_ok = actual["category"] == expected["category"]
        exact_ok = actual == expected

        category_correct += int(category_ok)
        exact_correct += int(exact_ok)

        print(f"Expected: {expected}")
        print(f"Actual:   {actual}")
        print(f"Category correct: {category_ok}")
        print(f"Exact match:      {exact_ok}")

    total = len(cases)
    attempted = total - failures

    print("\n========== EVALUATION SUMMARY ==========")
    print(f"Total cases:            {total}")
    print(f"Successful calls:       {attempted}")
    print(f"Failed calls:           {failures}")

    if attempted:
        print(f"Category accuracy:      {category_correct / total:.1%} of all cases")
        print(f"Exact-match accuracy:   {exact_correct / total:.1%} of all cases")

    if failures or exact_correct != total:
        sys.exit(1)


if __name__ == "__main__":
    main()
