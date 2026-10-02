import logging


class RequestContextFilter(
    logging.Filter
):

    def filter(
        self,
        record: logging.LogRecord,
    ) -> bool:

        if not hasattr(
            record,
            "request_id",
        ):
            record.request_id = "-"

        if not hasattr(
            record,
            "operation",
        ):
            record.operation = "-"

        return True


def configure_logging() -> None:

    handler = logging.StreamHandler()

    handler.setFormatter(
        logging.Formatter(
            "%(asctime)s | "
            "%(levelname)s | "
            "%(name)s | "
            "request_id=%(request_id)s | "
            "operation=%(operation)s | "
            "%(message)s"
        )
    )

    handler.addFilter(
        RequestContextFilter()
    )

    root_logger = logging.getLogger()

    root_logger.setLevel(
        logging.INFO
    )

    root_logger.handlers.clear()

    root_logger.addHandler(handler)