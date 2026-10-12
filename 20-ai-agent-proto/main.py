import sys

from ai_agent.runner import run_pipeline


def main() -> None:
    # Accept input from CLI argument or fallback to sample
    input_text = sys.argv[1] if len(sys.argv) > 1 else "My order #4821 has not arrived."
    output = run_pipeline(input_text)
    print(output)


if __name__ == "__main__":
    main()
