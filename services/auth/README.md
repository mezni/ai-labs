## Sequence Diagram: JWT Authentication Flow

''' mermaid
    sequenceDiagram
        participant C as Client
        participant A as Auth Service
        participant UM as User Memory
        participant JWT as JWT Service

        Note over C, JWT: 1. Registration Flow
        C->>A: POST /api/v1/auth/register
        A->>UM: Check if user exists
        UM-->>A: User not found
        A->>UM: Store new user
        A-->>C: 201 User created

        Note over C, JWT: 2. Login Flow
        C->>A: POST /api/v1/auth/login
        A->>UM: Validate credentials
        UM-->>A: User validated
        A->>JWT: Generate JWT token
        JWT-->>A: Token with claims
        A-->>C: 200 {token, username, role}

        Note over C, JWT: 3. Access Protected Route
        C->>A: GET /api/v1/auth/profile
        Note right of C: Authorization: Bearer <token>
        A->>JWT: Verify & decode token
        JWT-->>A: Claims {username, role}
        A->>UM: Verify user still active
        UM-->>A: User active
        A-->>C: 200 User profile

        Note over C, JWT: 4. Token Validation
        C->>A: GET /api/v1/auth/validate
        Note right of C: Authorization: Bearer <token>
        A->>JWT: Verify token
        JWT-->>A: Valid claims
        A-->>C: 200 Token valid

        Note over C, JWT: 5. Admin Only Route
        C->>A: GET /api/v1/auth/users
        Note right of C: Authorization: Bearer <token>
        A->>JWT: Verify token
        JWT-->>A: Claims {username, role=user}
        A-->>C: 403 Insufficient permissions



## Integration with Your Network Service:

''' mermaid
    sequenceDiagram
        participant C as Client
        participant NS as Network Service
        participant AU as AuthenticatedUser<br/>Extractor
        participant DB as Database

        Note over C, DB: Create Network with Authentication
        C->>NS: POST /api/v1/networks
        Note right of C: Authorization: Bearer <token>
        
        NS->>AU: Extract user from token
        AU-->>NS: AuthenticatedUser {username, role}
        
        NS->>DB: INSERT INTO networks (... created_by)
        Note right of NS: created_by = username
        DB-->>NS: Network created
        NS-->>C: 201 Network with created_by

        Note over C, DB: Update Network with Authentication
        C->>NS: PUT /api/v1/networks/{id}
        Note right of C: Authorization: Bearer <token>
        
        NS->>AU: Extract user from token
        AU-->>NS: AuthenticatedUser {username, role}
        
        NS->>DB: UPDATE networks SET ... updated_by
        Note right of NS: updated_by = username
        DB-->>NS: Network updated
        NS-->>C: 200 Network with updated_by


# First login to get token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'

# Then create network with token
curl -X POST http://localhost:8080/api/v1/networks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "My Network",
    "type": "company",
    "contact_email": "admin@example.com",
    "phone_number": "+1234567890",
    "address": "123 Main St"
  }'





  ┌─────────────────┐    JWT Token     ┌──────────────────┐
│   Auth Service  │ ────────────────►│ Network Service  │
│   (port 8081)   │                  │   (port 8080)    │
│                 │                  │                  │
│ • /auth/login   │                  │ • /networks      │
│ • /auth/register│                  │ • /networks/{id} │
└─────────────────┘                  └──────────────────┘
         │                                      │
         ▼                                      ▼
   In-memory users                         PostgreSQL DB