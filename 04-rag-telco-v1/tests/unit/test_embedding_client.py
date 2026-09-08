import json

import httpx

from src.llm import EmbeddingClient


def test_embedding_client():

    embedding = [0.1, 0.2, 0.3]

    def handler(request: httpx.Request) -> httpx.Response:
        assert request.method == "POST"

        body = json.loads(request.content)

        assert body["input"] == "international roaming"

        return httpx.Response(
            200,
            json={
                "data": [
                    {
                        "index": 0,
                        "embedding": embedding,
                    }
                ]
            },
        )

    transport = httpx.MockTransport(handler)

    client = EmbeddingClient(
        model="test-model",
        transport=transport,
    )

    result = client.embed("international roaming")

    assert result == embedding

    client.close()

def test_embedding_client_embed_many():

    def handler(request: httpx.Request) -> httpx.Response:
        body = json.loads(request.content)

        assert body["input"] == [
            "roaming",
            "billing",
        ]

        return httpx.Response(
            200,
            json={
                "data": [
                    {
                        "index": 0,
                        "embedding": [0.1, 0.2],
                    },
                    {
                        "index": 1,
                        "embedding": [0.3, 0.4],
                    },
                ]
            },
        )

    transport = httpx.MockTransport(handler)

    client = EmbeddingClient(
        model="test-model",
        transport=transport,
    )

    result = client.embed_many(
        [
            "roaming",
            "billing",
        ]
    )

    assert result == [
        [0.1, 0.2],
        [0.3, 0.4],
    ]

    client.close()