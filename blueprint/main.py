import json
import os

from dotenv import load_dotenv
from openai import OpenAI

load_dotenv()

PROMPT = """You are a product naming expert for software projects.
Given the project idea below, generate 5 distinct, brandable project names.

Idea: {idea}

Rules:
- Short and memorable; prefer real-word or portmanteau constructions.
- No trademarked brands, offensive terms, or jargon.
- Respond with JSON only: {{"names": ["Name1", "Name2", ...]}}"""


def _client() -> OpenAI:
    api_key = os.getenv("OPENROUTER_API_KEY")
    if not api_key:
        raise SystemExit("OPENROUTER_API_KEY is not set. Add it to your .env file.")
    return OpenAI(api_key=api_key, base_url="https://openrouter.ai/api/v1")


def generate_names(project_idea):
    model = os.getenv("CATALYST_MODEL", "minimax/minimax-m3:free")
    client = _client()

    response = client.chat.completions.create(
        model=model,
        temperature=0.3,
        response_format={"type": "json_object"},
        messages=[
            {
                "role": "user",
                "content": PROMPT.format(idea=project_idea[:1000].strip()),
            }
        ],
    )
    payload = json.loads(response.choices[0].message.content)
    return payload["names"]


def get_project_idea():
    return input("Enter your IT project idea: ")


def display_names(names):
    print()
    print("Generated project names:")
    print()

    for index, name in enumerate(names, start=1):
        print(f"{index}. {name}")


def main():
    idea = get_project_idea()

    names = generate_names(idea)

    display_names(names)


if __name__ == "__main__":
    main()