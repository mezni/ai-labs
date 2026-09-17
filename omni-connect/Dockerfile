# Omni-Connect Retail Copilot - container build.
# Streamlit portal + RAG + specialist agents. The embedding/reranker/SDK
# wheels are pulled by pip at build time; .env keys (ANTHROPIC_API_KEY /
# LLM_BASE_URL / HF_TOKEN) are passed through at runtime via docker compose.
FROM python:3.13-slim

WORKDIR /app

# System deps for faiss-cpu / numpy wheels build cleanly on slim images.
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 8501

# Basic container health check - confirms the Streamlit process is serving,
# not that the copilot workflow itself is functional (that needs configured
# ANTHROPIC_API_KEY / LLM_BASE_URL at runtime, passed via env_file .env).
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8501/_stcore/health')" || exit 1

CMD ["streamlit", "run", "app/portal.py", "--server.port=8501", "--server.address=0.0.0.0"]