def get_project_idea():
    idea = input("Enter your IT project idea: ")
    return idea


def main():
    idea = get_project_idea()

    print()
    print("Project idea:")
    print(idea)


if __name__ == "__main__":
    main()