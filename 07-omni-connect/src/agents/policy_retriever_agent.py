"""
Policy Retriever Agent - invokes the RAG pipeline over the omni-connect
knowledge base to ground answers in exact policy details.

Adapted from the bootcamp lending_policy_agent.py.txt pattern: pulls the
relevant policy passages from the FAISS + BM25 hybrid Retriever, collects
per-source citations, and asks the LLM to explain how the retrieved policy
applies to the question. The persisted index (data/kb_store/) is loaded on
first use; if it has not been built, the index is built lazily from the
corpus path in config/llm_config.yaml.
"""
from pathlib import Path
from typing import Any, Dict, List, Optional

from src.knowledge.knowledge_base import KnowledgeBase
from src.knowledge.retriever import Retriever
from src.llm.llm_client import LLMClient
from src.llm.prompt_manager import PromptManager
from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

ROOT = Path(__file__).resolve().parents[2]
INDEX_STEM = ROOT / "data" / "kb_store" / "vector_index"
CONFIG = load_yaml_config("config/llm_config.yaml")


class PolicyRetrieverAgent:
    """Answers policy questions via RAG retrieval over the knowledge base,
    returning grounded citations alongside the LLM summary."""

    def __init__(
        self,
        llm_client: Optional[LLMClient] = None,
        prompt_manager: Optional[PromptManager] = None,
        knowledge_base: Optional[KnowledgeBase] = None,
    ) -> None:
        self.llm = llm_client or LLMClient()
        self.prompts = prompt_manager or PromptManager()
        self.kb = knowledge_base or KnowledgeBase()
        self._ready = False
        retrieval = CONFIG.get("retrieval", {})
        self.corpus_path = retrieval.get("corpus", {}).get("path", "data/knowledge_base")

    def _ensure_index(self) -> None:
        """Load the persisted index on first use; build it lazily if absent."""
        if self._ready:
            return
        if (Path(f"{INDEX_STEM}.index").exists() and Path(f"{INDEX_STEM}.docs").exists()):
            self.kb.load_index(str(INDEX_STEM))
            logger.info("Loaded persisted index (%d documents)", len(self.kb.documents))
        else:
            docs = self.kb.load_documents(self.corpus_path)
            if not docs:
                raise FileNotFoundError(f"No corpus documents found under {self.corpus_path}")
            self.kb.build_index(docs)
            logger.info("Built index over %d documents", len(docs))
        self._ready = True

    def retrieve(
        self,
        query: str,
        top_k: Optional[int] = None,
        threshold: float = 0.0,
    ) -> List[Dict[str, Any]]:
        """Return raw retrieval hits (content + source + score) for `query`."""
        self._ensure_index()
        retriever = Retriever(self.kb)
        return retriever.retrieve(query, top_k=top_k, threshold=threshold)

    def analyze(
        self,
        query: str,
        context: Optional[Dict[str, Any]] = None,
        top_k: Optional[int] = None,
        threshold: float = 0.0,
    ) -> Dict[str, Any]:
        """Answer `query` grounded in retrieved policy excerpts, with citations."""
        results = self.retrieve(query, top_k=top_k, threshold=threshold)

        policy_excerpts = "\n\n".join(r["content"] for r in results) or "No matching policy excerpts found."
        citations = [
            {"source": r["source"], "category": r["category"], "score": r["score"]}
            for r in results
        ]

        customer_context = ""
        if context:
            customer_context = "\n".join(f"{key}: {value}" for key, value in context.items())

        prompt = self.prompts.get_prompt("lending_policy_prompt")
        user_message = self.prompts.format_prompt(
            prompt.get("user_template", ""),
            query=query,
            customer_context=customer_context,
            policy_excerpts=policy_excerpts,
        )
        summary = self.llm.generate([
            {"role": "system", "content": prompt.get("system", "")},
            {"role": "user", "content": user_message},
        ])

        return {
            "agent": "policy_retriever",
            "query": query,
            "summary": summary,
            "citations": citations,
        }