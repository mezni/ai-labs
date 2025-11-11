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