# Corpus

Synthetic documents for the fictional operator Aether Wireless.
Nothing here is real operator content.

## Layout

- `raw/<domain>/` holds the source documents.
- `manifest.json` describes every document version.

## Manifest fields

| Field | Meaning |
|---|---|
| doc_id | Stable document ID, e.g. AW-ROA-002 |
| version | Integer version of the document |
| domain | billing, roaming, plans or network |
| path | File path relative to `data/` |
| content_type | MIME type, used later to pick a parser |
| plan_types | postpaid, prepaid, or both |
| regions | ALL or a list of region codes |
| effective_from / effective_to | Validity window (ISO dates, `null` = open-ended) |
| status | current or superseded |