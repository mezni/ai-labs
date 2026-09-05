from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

from app.services.chat import generate

app = FastAPI(title="Store Advisor")


class ChatRequest(BaseModel):
    message: str
    model: str | None = None


class ChatResponse(BaseModel):
    reply: str


@app.get("/health")
def health() -> dict:
    return {"status": "ok"}


@app.post("/chat", response_model=ChatResponse)
def chat(req: ChatRequest) -> ChatResponse:
    try:
        kwargs = {"model": req.model} if req.model else {}
        reply = generate(
            messages=[{"role": "user", "content": req.message}],
            **kwargs,
        )
    except Exception:
        raise HTTPException(status_code=502, detail="LLM request failed")
    return ChatResponse(reply=reply)