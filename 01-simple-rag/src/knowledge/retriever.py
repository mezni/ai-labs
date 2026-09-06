from src.knowledge.embeddings import embed
from src.knowledge.knowledge_base import KnowledgeBase
from src.utils.config_loader import load_yaml_config
from src.utils.logger import get_logger

log = get_logger(__name__)

_config = load_yaml_config("config/llm_config.yaml")
_chunk_cfg = _config["chunking"]

RAG_SYSTEM_PROMPT = """\
You are a helpful assistant. Answer the user's question ONLY using the context provided below.
If the context does not contain enough information to answer, state "I do not have enough information."

Context:
---
{context}

User Question: {question}
"""


class Retriever:
    def __init__(self, kb: KnowledgeBase, top_k: int = 5) -> None:
        self.kb = kb
        self.top_k = top_k

    def retrieve(self, query: str) -> list[dict[str, str]]:
        query_embedding = embed([query])[0]
        return self.kb.search(query_embedding, top_k=self.top_k)

    def build_prompt(self, query: str) -> str:
        results = self.retrieve(query)
        context = "\n---\n".join(r["text"] for r in results)
        return RAG_SYSTEM_PROMPT.format(context=context, question=query)