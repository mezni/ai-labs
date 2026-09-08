import logging

from fastapi import APIRouter

from app.core.config import get_settings
from app.core.db import check_database_ready

logger = logging.getLogger(__name__)

router = APIRouter()


@router.get("")
def health() -> dict:
    logger.info("Liveness check")
    return {"status": "UP"}


@router.get("/live")
def health_live() -> dict:
    logger.info("Liveness detail check")
    return {"status": "ALIVE"}


@router.get("/ready")
def health_ready() -> dict:
    db_ok = check_database_ready()
    settings = get_settings()
    llm_status = "UP" if settings.openrouter_api_key else "DEGRADED"
    status = "READY" if db_ok else "NOT_READY"

    logger.info(
        "Readiness check database=%s llm=%s status=%s",
        "UP" if db_ok else "DOWN",
        llm_status,
        status,
    )
    return {
        "status": status,
        "database": "UP" if db_ok else "DOWN",
        "llm": llm_status,
    }
