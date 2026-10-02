import logging


class StructuredLogger:

    def __init__(
        self,
        logger: logging.Logger,
    ) -> None:

        self.logger = logger

    def info(
        self,
        message: str,
        *,
        request_id: str = "-",
        operation: str = "-",
    ) -> None:

        self.logger.info(
            message,
            extra={
                "request_id": request_id,
                "operation": operation,
            },
        )

    def error(
        self,
        message: str,
        *,
        request_id: str = "-",
        operation: str = "-",
    ) -> None:

        self.logger.error(
            message,
            extra={
                "request_id": request_id,
                "operation": operation,
            },
        )

    def exception(
        self,
        message: str,
        *,
        request_id: str = "-",
        operation: str = "-",
    ) -> None:

        self.logger.exception(
            message,
            extra={
                "request_id": request_id,
                "operation": operation,
            },
        )