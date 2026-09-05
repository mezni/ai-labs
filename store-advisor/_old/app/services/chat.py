import os

from dotenv import load_dotenv
from openai import OpenAI

load_dotenv()

DEFAULT_MODEL = os.getenv("OPENROUTER_MODEL", "minimax/minimax-m3:free")


def get_client() -> OpenAI:
    api_key = os.getenv("OPENROUTER_API_KEY")
    if not api_key:
        raise RuntimeError("OPENROUTER_API_KEY not set in .env")
    return OpenAI(
        base_url="https://openrouter.ai/api/v1",
        api_key=api_key,
    )


def generate(messages: list[dict], model: str = DEFAULT_MODEL, client: OpenAI | None = None) -> str:
    client = client or get_client()
    response = client.chat.completions.create(model=model, messages=messages)
    content = response.choices[0].message.content
    if content is None:
        raise RuntimeError("Empty response from model")
    return content


def cli() -> None:
    client = get_client()
    messages: list[dict] = []
    print(f"Model: {os.getenv('OPENROUTER_MODEL', DEFAULT_MODEL)} — type 'exit' or Ctrl+D to quit\n")

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
            reply = generate(messages, client=client)
            messages.append({"role": "assistant", "content": reply})
            print(f"\nAssistant: {reply}\n")
        except Exception as e:
            print(f"\nError: {e}\n")
            messages.pop()


if __name__ == "__main__":
    cli()