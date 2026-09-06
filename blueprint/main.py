def get_project_idea():
    return input("Enter your IT project idea: ")


def generate_names(project_idea):
    return [
        "AgentPulse",
        "AgentScope",
        "AgentRadar",
        "ModelWatch",
        "AIOpsLens",
    ]


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