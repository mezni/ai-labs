import sys

from application import create_application
from logging_config import configure_logging
from schemas import GenerateBlueprintRequest


class CLI:
    def __init__(self) -> None:
        self.application = create_application()

    def run(self, project_idea: str) -> None:
        request = GenerateBlueprintRequest(
            project_idea=project_idea
        )

        response = self.application.generate_blueprint(request)

        self._display_response(response)

    def _display_response(self, response) -> None:
        print()
        print(f"Project Name: {response.blueprint.project_name}")
        print()
        print(
            f"Business Outcome: "
            f"{response.blueprint.business_outcome}"
        )
        print()
        print(f"Request ID: {response.telemetry.request_id}")
        print(
            f"Latency: "
            f"{response.telemetry.latency_seconds:.2f}s"
        )


def main() -> None:

    configure_logging()

    if len(sys.argv) < 2:
        raise SystemExit(
            'Usage: uv run python app.py "<project idea>"'
        )

    cli = CLI()
    cli.run(sys.argv[1])


if __name__ == "__main__":
    main()