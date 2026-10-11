import re

ARTICLES = [
    {
        "id": "returns-001",
        "title": "Return and Refund Policy",
        "content": (
            "Customers may request a return within 30 days "
            "of delivery. Items must be unused and in their "
            "original packaging. Refund processing begins "
            "after the returned item is received and inspected."
        ),
        "keywords": ["return", "refund", "money", "exchange"],
    },
    {
        "id": "shipping-001",
        "title": "Shipping Information",
        "content": (
            "Standard shipping typically takes 3 to 5 business "
            "days after an order ships. Delivery estimates are "
            "not guaranteed and may change due to carrier delays."
        ),
        "keywords": ["shipping", "delivery", "arrive", "carrier"],
    },
    {
        "id": "account-001",
        "title": "Update Account Email",
        "content": (
            "Customers can update their email address in "
            "account settings. They may need to verify the "
            "new email address before the change takes effect."
        ),
        "keywords": ["account", "email", "profile", "settings"],
    },
]


def search_knowledge_base(query: str) -> list[dict[str, str]]:
    """Return articles ranked by simple keyword overlap."""
    if not isinstance(query, str) or not query.strip():
        raise ValueError("Search query must not be empty.")

    query_terms = set(re.findall(r"[a-z0-9]+", query.lower()))
    ranked = []

    for article in ARTICLES:
        searchable_text = " ".join(
            [
                article["title"],
                article["content"],
                " ".join(article["keywords"]),
            ]
        ).lower()

        article_terms = set(re.findall(r"[a-z0-9]+", searchable_text))
        score = len(query_terms & article_terms)

        if score > 0:
            ranked.append((score, article))

    ranked.sort(key=lambda item: item[0], reverse=True)

    return [
        {
            "id": article["id"],
            "title": article["title"],
            "content": article["content"],
        }
        for _, article in ranked[:3]
    ]
