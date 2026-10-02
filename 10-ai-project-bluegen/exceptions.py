from failure import Failure


class ProjectGenerationError(Exception):
    """Raised when project blueprint generation fails."""

    def __init__(
        self,
        message: str,
        failure: Failure | None = None,
    ) -> None:

        super().__init__(message)

        self.failure = failure
