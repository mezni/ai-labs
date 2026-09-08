# API_SPEC.md

# Telecom Policy Assistant - API Specification

Version: 1.0  
Status: Draft  
Owner: Platform Engineering Team

---

# 1. Purpose

This document defines the REST API contract for the Telecom Policy Assistant. The API enables:

- Natural language policy questions
- Conversation management
- Document management
- Feedback collection
- Observability integration
- FinOps reporting

The API follows REST principles and is implemented using FastAPI.

---

# 2. API Principles

## API-001 RESTful

All endpoints shall follow REST conventions.

---

## API-002 Versioned

All public APIs shall be versioned.

Example:

```http
/api/v1
```

---

## API-003 Secure

All endpoints require authentication unless explicitly documented.

---

## API-004 Observable

Every request must generate:

```text
Trace ID
Request ID
Audit Event
```

---

## API-005 Consistent Responses

All APIs must follow a common response structure.

---

# 3. Base URLs

All endpoints documented in this specification are relative to the base URL. For example, `GET /health` resolves to `http://localhost:8000/api/v1/health`.

## Development

```text
http://localhost:8000/api/v1
```

---

## QA

```text
https://qa-assistant.company.com/api/v1
```

---

## Production

```text
https://assistant.company.com/api/v1
```

---

# 4. Authentication

## Supported Providers

```text
Microsoft Entra ID

OIDC

Keycloak
```

---

## Request Header

```http
Authorization: Bearer <access_token>
```

---

## Example

```http
GET /api/v1/conversations

Authorization: Bearer eyJ...
```

---

# 5. Request Headers

## Required

```http
Authorization
Content-Type
```

---

## Optional

```http
X-Correlation-ID
X-Request-ID
```

---

# 6. Common Response Schema

## Success Response

```json
{
  "success": true,
  "data": {}
}
```

---

## Error Response

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Question cannot be empty"
  }
}
```

---

# 7. Chat APIs

## Ask a Question

### Endpoint

```http
POST /chat/ask
```

---

### Description

Submit a policy question to the assistant.

---

### Request

```json
{
  "question": "Can I use my mobile plan in Spain?",
  "conversation_id": "optional-uuid"
}
```

---

### Response

```json
{
  "success": true,
  "data": {
    "question_id": "uuid",
    "answer_id": "uuid",
    "answer": "According to the Roaming Policy, Spain belongs to Zone 1.",
    "confidence": 0.92,
    "sources": [
      {
        "document": "Roaming Policy V3",
        "page": 34,
        "section": "Zone 1 Europe"
      }
    ],
    "latency_ms": 2175
  }
}
```

---

### Status Codes

```text
200 OK

400 Bad Request

401 Unauthorized

429 Too Many Requests

500 Internal Server Error
```

---

## Stream Response (Future)

### Endpoint

```http
POST /chat/stream
```

---

### Response

```text
Server-Sent Events (SSE)
```

---

# 8. Conversation APIs

## Create Conversation

### Endpoint

```http
POST /conversations
```

---

### Request

```json
{
  "title": "Roaming Questions"
}
```

---

### Response

```json
{
  "success": true,
  "data": {
    "conversation_id": "uuid",
    "title": "Roaming Questions"
  }
}
```

---

## List Conversations

### Endpoint

```http
GET /conversations
```

---

### Response

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "title": "Roaming Questions",
      "updated_at": "2026-09-08T10:00:00Z"
    }
  ]
}
```

---

## Get Conversation

### Endpoint

```http
GET /conversations/{conversation_id}
```

---

### Response

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "title": "Roaming Questions",
    "messages": []
  }
}
```

---

## Delete Conversation

### Endpoint

```http
DELETE /conversations/{conversation_id}
```

---

### Response

```json
{
  "success": true
}
```

---

# 9. Feedback APIs

## Submit Feedback

### Endpoint

```http
POST /feedback
```

---

### Request

```json
{
  "answer_id": "uuid",
  "rating": "HELPFUL",
  "comment": "Good explanation and references."
}
```

---

### Allowed Ratings

```text
HELPFUL

NOT_HELPFUL
```

---

### Response

```json
{
  "success": true,
  "data": {
    "feedback_id": "uuid"
  }
}
```

---

# 10. Search APIs

## Search Knowledge Base

### Endpoint

```http
POST /search
```

---

### Description

Retrieve source content without generating an answer.

---

### Request

```json
{
  "query": "Spain roaming",
  "category": "ROAMING"
}
```

---

### Response

```json
{
  "success": true,
  "data": {
    "results": [
      {
        "document": "Roaming Policy V3",
        "page": 34,
        "section": "Zone 1 Europe",
        "score": 0.94
      }
    ]
  }
}
```

---

# 11. Document APIs

## Upload Document

### Endpoint

```http
POST /documents/upload
```

---

### Content Type

```http
multipart/form-data
```

---

### Request

```text
file=RoamingPolicyV3.pdf

category=ROAMING

version=v3
```

---

### Response

```json
{
  "success": true,
  "data": {
    "document_id": "uuid",
    "status": "INGESTION_STARTED"
  }
}
```

---

## List Documents

### Endpoint

```http
GET /documents
```

---

### Response

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "Roaming Policy",
      "version": "v3",
      "status": "ACTIVE"
    }
  ]
}
```

---

## Get Document

### Endpoint

```http
GET /documents/{document_id}
```

---

### Response

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Roaming Policy",
    "category": "ROAMING",
    "version": "v3",
    "status": "ACTIVE"
  }
}
```

---

## Activate Version

### Endpoint

```http
POST /documents/{document_id}/activate
```

---

### Response

```json
{
  "success": true,
  "data": {
    "status": "ACTIVE"
  }
}
```

---

## Reindex Document

### Endpoint

```http
POST /documents/{document_id}/reindex
```

---

### Response

```json
{
  "success": true,
  "data": {
    "status": "REINDEX_STARTED"
  }
}
```

---

# 12. Administrative APIs

## Full Reindex

### Endpoint

```http
POST /admin/reindex
```

---

### Role

```text
ADMIN
```

---

### Response

```json
{
  "success": true,
  "data": {
    "job_id": "uuid"
  }
}
```

---

## Rebuild Embeddings

### Endpoint

```http
POST /admin/embeddings/rebuild
```

---

### Response

```json
{
  "success": true,
  "data": {
    "job_id": "uuid"
  }
}
```

---

# 13. Reporting APIs

## Cost Report

### Endpoint

```http
GET /reports/costs
```

---

### Query Parameters

```text
from
to
```

---

### Example

```http
GET /reports/costs?from=2026-09-01&to=2026-09-30
```

---

### Response

```json
{
  "success": true,
  "data": {
    "total_cost": 120.50,
    "questions": 5120,
    "avg_cost_per_question": 0.0235
  }
}
```

---

## Usage Report

### Endpoint

```http
GET /reports/usage
```

---

### Response

```json
{
  "success": true,
  "data": {
    "users": 155,
    "questions": 5120,
    "active_stores": 8
  }
}
```

---

## Model Utilization Report

### Endpoint

```http
GET /reports/models
```

---

### Response

```json
{
  "success": true,
  "data": [
    {
      "model": "minimax",
      "requests": 5100,
      "tokens": 1725000,
      "cost": 120.50
    }
  ]
}
```

---

# 14. Observability APIs

## Health Check

### Endpoint

```http
GET /health
```

---

### Response

```json
{
  "status": "UP"
}
```

---

## Readiness Check

### Endpoint

```http
GET /health/ready
```

---

### Response

```json
{
  "status": "READY",
  "database": "UP",
  "llm": "UP"
}
```

---

## Liveness Check

### Endpoint

```http
GET /health/live
```

---

### Response

```json
{
  "status": "ALIVE"
}
```

---

# Definition of Success

The REST API provides a stable, versioned, secure, and observable contract that enables users to ask policy questions, manage conversations and documents, collect feedback, and report costs with consistent responses and full traceability.
