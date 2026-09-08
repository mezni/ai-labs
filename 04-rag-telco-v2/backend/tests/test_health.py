from fastapi.testclient import TestClient

from app.main import app

client = TestClient(app)


def test_health_liveness() -> None:
    response = client.get("/api/v1/health")
    assert response.status_code == 200
    assert response.json() == {"status": "UP"}


def test_health_live() -> None:
    response = client.get("/api/v1/health/live")
    assert response.status_code == 200
    assert response.json() == {"status": "ALIVE"}


def test_health_ready_shape() -> None:
    response = client.get("/api/v1/health/ready")
    assert response.status_code == 200
    body = response.json()
    assert "status" in body
    assert "database" in body
    assert "llm" in body
    assert body["database"] in {"UP", "DOWN"}
    assert body["llm"] in {"UP", "DEGRADED", "DOWN"}
    assert body["status"] in {"READY", "NOT_READY"}
