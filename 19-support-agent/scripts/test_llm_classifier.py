import json
import sys

from support_agent.llm_classifier import classify_ticket


TEST_TICKETS = [
    "Where is order #4821?",
    "My package never arrived.",
    "I want my money back.",
    "How do I change my email?",
    "I want a refund for order #4821.",
]


def main() -> None:
    passed = 0
    failed = 0

    for ticket in TEST_TICKETS:
        print(f"\nTicket: {ticket}")

        try:
            result = classify_ticket(ticket)
            print(
                json.dumps(
                    result.model_dump(),
                    indent=2,
                    ensure_ascii=False,
                )
            )
            passed += 1

        except Exception as exc:
            failed += 1
            print(f"ERROR: {type(exc).__name__}: {exc}")

    print("\n" + "=" * 40)
    print(f"Total: {len(TEST_TICKETS)}")
    print(f"Successful calls: {passed}")
    print(f"Failed calls: {failed}")

    if failed:
        sys.exit(1)


if __name__ == "__main__":
    main()
