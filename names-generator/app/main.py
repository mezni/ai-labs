import sys

from dotenv import load_dotenv

from agents.name_generator_agent import NameGeneratorAgent
from llm.llm_client import LLMConfigError

load_dotenv()


def get_project_idea():
    return input("Enter your IT project idea: ")


def display_names(names):
    print()
    print("Generated project names:")
    print()

    for index, candidate in enumerate(names, start=1):
        print(f"{index}. {candidate.name}")
        print(f"   {candidate.description}")
        print(f"   Why: {candidate.reason}")
        print()


def main():
    idea = get_project_idea()

    try:
        agent = NameGeneratorAgent()
        names = agent.generate(idea)
    except (LLMConfigError, ValueError, RuntimeError) as error:
        print(f"Error: {error}")
        sys.exit(1)

    display_names(names)


if __name__ == "__main__":
    main()