from fastapi import FastAPI

from app.api.router import api_router
from app.core.config import get_settings
from app.core.observability import configure_logging


def create_app() -> FastAPI:
    settings = get_settings()
    configure_logging()
    app = FastAPI(title=settings.app_name)
    app.include_router(api_router, prefix="/api/v1")
    return app


app = create_app()
