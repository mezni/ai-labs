# AI Experiments

Mini-projects to learn AI, one step at a time. Each project is self-contained and builds on the previous ones.

> **Aether Wireless** — a sleek, cloud-native, next-gen telecom provider. All projects below will be themed around this branding.

| # | Project | Description | Stack |
|---|---------|-------------|-------|
| 01 | simple-rag | A simple Retrieval-Augmented Generation system that chunks documents, indexes them with FAISS, and answers questions using an LLM | Python, Streamlit, FAISS |
| 02 | prompt-arena | Side-by-side A/B testing of prompts and models to learn prompt engineering | Python, Streamlit, LangChain |
| 03 | embedding-visualizer | Visualize text embeddings with PCA/t-SNE on a toy dataset | Python, scikit-learn, Plotly |
| 04 | chatbot-memory | Chatbot with conversation memory (session + long-term vector memory) | Python, Streamlit, FAISS |
| 05 | image-classifier | Train a CNN on a small dataset (MNIST/CIFAR) with experiment tracking | Python, PyTorch, MLflow |
| 06 | llm-finetune | Fine-tune a small model (LLaMA/GPT-2) on a custom dataset with LoRA | Python, HuggingFace, PEFT |
| 07 | rag-evaluator | Evaluate RAG quality (retrieval hits, answer faithfulness) across configs | Python, RAGAS, LangChain |
| 08 | model-comparison | Benchmark open-source LLMs locally on a custom eval set | Python, Ollama, vLLM |
| 09 | agent-calculator | Tool-using agent that solves math problems with a calculator tool | Python, LangGraph, Streamlit |
| 10 | voice-assistant | Speech-to-text → LLM → text-to-speech voice assistant | Python, Whisper, Streamlit |

## Structure

```
01-simple-rag/    <-- project folder
  app.py          <-- main entry point
  requirements.txt
  README.md       <-- per-project explainer
```

## How to run

```bash
cd 01-simple-rag
pip install -r requirements.txt
streamlit run app.py
```