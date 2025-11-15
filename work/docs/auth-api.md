# 🔐 Auth Service

A robust authentication microservice built with Rust and Actix-web, providing JWT-based authentication, user management, and role-based access control.

## 🚀 Features

- **JWT Authentication** - Secure token-based authentication
- **User Management** - Register, login, and user profile management
- **Role-Based Access Control** - Admin and user roles with appropriate permissions
- **RESTful API** - Clean, well-documented REST endpoints
- **OpenAPI Documentation** - Interactive API documentation with Swagger UI
- **Docker Support** - Easy deployment with Docker and Docker Compose
- **Environment Configuration** - Flexible configuration via environment variables

## 📋 API Endpoints

| Method | Endpoint | Auth Required | Description |
|--------|----------|---------------|-------------|
| `GET` | `/api/v1/health` | ❌ | Service health check |
| `POST` | `/api/v1/auth/register` | ❌ | Register new user |
| `POST` | `/api/v1/auth/login` | ❌ | User login (get JWT token) |
| `GET` | `/api/v1/auth/profile` | ✅ | Get current user profile |
| `GET` | `/api/v1/auth/validate` | ✅ | Validate JWT token |
| `GET` | `/api/v1/auth/users` | ✅ | List all users (Admin only) |

# 🔐 Auth Service API Response Codes

## 📋 Overview

| Code | Status | Description |
|------|--------|-------------|
| `200` | OK | Request succeeded |
| `201` | Created | Resource created successfully |
| `400` | Bad Request | Invalid input data |
| `401` | Unauthorized | Authentication required or failed |
| `403` | Forbidden | Insufficient permissions |
| `404` | Not Found | Resource not found |
| `409` | Conflict | Resource already exists |
| `500` | Internal Server Error | Server-side error |

---

## 🔍 Detailed Endpoint Responses

### 1. Health Check
**GET** `/api/v1/health`

**Action**: Check service status

**Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

"Auth service is healthy"
```

### 2. Register User
**POST** `/api/v1/auth/register`

**Action**: Create new user account
#### Request Body:
```http
{
  "username": "newuser",
  "password": "password123",
  "role": "user"
}
```

#### Success Response:

```http

HTTP/1.1 201 Created
Content-Type: application/json
{
  "username": "newuser",
  "role": "user",
  "is_active": true
}
```

#### Error Responses:

```http

HTTP/1.1 400 Bad Request
Content-Type: application/json
"Validation error"
```

```http

HTTP/1.1 409 Conflict
Content-Type: application/json
"User already exists"
```

```http

HTTP/1.1 500 Internal Server Error
Content-Type: application/json
"Internal server error"
```


### 3. User Login
**POST** `/api/v1/auth/login`


**Action**: Authenticate user and return JWT token
#### Request Body:

```http

{
  "username": "admin",
  "password": "admin123"
}
```

#### Success Response:

```http

HTTP/1.1 200 OK
Content-Type: application/json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "username": "admin",
  "role": "admin"
}
```

#### Error Response:

```http

HTTP/1.1 401 Unauthorized
Content-Type: application/json
"Invalid credentials"
```