import os

import argparse
from dotenv import load_dotenv
from openai import OpenAI

load_dotenv()

DEFAULT_MODEL = "openrouter/free"


def get_client() -> OpenAI:
    api_key = os.getenv("OPENROUTER_API_KEY")
    if not api_key:
        raise SystemExit("OPENROUTER_API_KEY not set in .env")
    return OpenAI(
        base_url="https://openrouter.ai/api/v1",
        api_key=api_key,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description="Simple chat with OpenRouter free models")
    parser.add_argument("--model", default=DEFAULT_MODEL, help="OpenRouter model ID (default: %(default)s)")
    args = parser.parse_args()

    client = get_client()
    messages: list[dict] = []
    print(f"Model: {args.model} — type 'exit' or Ctrl+D to quit\n")

    while True:
        try:
            user_input = input("You: ").strip()
        except (EOFError, KeyboardInterrupt):
            print()
            break
        if not user_input:
            continue
        if user_input.lower() in {"exit", "quit"}:
            break

        messages.append({"role": "user", "content": user_input})
        try:
            response = client.chat.completions.create(
                model=args.model,
                messages=messages,
            )
            reply = response.choices[0].message.content
            messages.append({"role": "assistant", "content": reply})
            print(f"\nAssistant: {reply}\n")
        except Exception as e:
            print(f"\nError: {e}\n")
            messages.pop()


if __name__ == "__main__":
    main()