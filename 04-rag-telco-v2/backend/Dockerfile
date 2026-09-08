FROM python:3.11-slim
WORKDIR /app

COPY pyproject.toml ./
RUN pip install --no-cache-dir \
    "fastapi>=0.115" \
    "uvicorn[standard]>=0.30" \
    "sqlalchemy>=2.0" \
    "alembic>=1.13" \
    "psycopg2-binary>=2.9" \
    "pgvector>=0.3" \
    "pydantic-settings>=2.2"

COPY app ./app
EXPOSE 8000
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"]