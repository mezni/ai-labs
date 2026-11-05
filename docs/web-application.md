# Web Application

## 1. Purpose

The `bio-com` web application provides the user interface for the e-commerce platform.

Customers use it to browse products, manage their accounts, place orders, make payments, and track deliveries.

Administrators use it to manage products, inventory, orders, customers, and other business operations.

The frontend is developed with **TypeScript and React**. It communicates with the Rust backend through REST APIs.

The web application is responsible for displaying information and collecting user actions. Business rules remain in the Rust backend.

## 2. Technology Stack

| Technology | Purpose |
|---|---|
| TypeScript | Application programming language |
| React | User interface library |
| Vite | Development server and build tool |
| React Router | Client-side navigation |
| TanStack Query | Server state and API data management |
| React Hook Form | Form management |
| Zod | Form and data validation |
| Tailwind CSS | Styling |
| Vitest | Unit testing |
| React Testing Library | Component testing |
| Playwright | End-to-end testing |

The frontend should remain lightweight. We do not need a complex frontend framework or additional infrastructure at this stage.

## 3. Main Features

### 3.1 Customer Application

Customers can:

- Register and log in.
- View and update their profile.
- Browse product categories.
- Search and filter products.
- View product details.
- Manage their shopping cart.
- Place orders.
- Complete checkout.
- View order history.
- Track shipments.
- View payment and refund information.

### 3.2 Administration Application

Administrators can:

- View the administration dashboard.
- Manage products and categories.
- Manage product variants.
- Manage inventory.
- View and manage orders.
- View customer information.
- Manage refunds.
- Monitor business operations.

Access to administrative features depends on the permissions assigned to the authenticated user.

## 4. Frontend Architecture

The frontend follows a feature-based structure.

Each feature contains the components and logic related to a specific business capability.

We do not need to reproduce the backend DDD architecture inside React. The frontend has its own responsibilities and boundaries.

The main architectural layers are:

**Presentation**

Contains pages, layouts, reusable components, and user interactions.

**Application**

Coordinates frontend workflows, such as checkout and account management.

**API**

Handles communication with the Rust backend.

**Shared**

Contains reusable utilities, types, UI components, and common configuration.

## 5. Project Structure

```text
apps/
└── web/
    ├── public/
    │
    ├── src/
    │   ├── app/
    │   │   ├── router/
    │   │   ├── providers/
    │   │   └── layouts/
    │   │
    │   ├── features/
    │   │   ├── auth/
    │   │   │   ├── components/
    │   │   │   ├── pages/
    │   │   │   ├── hooks/
    │   │   │   └── services/
    │   │   │
    │   │   ├── catalog/
    │   │   ├── cart/
    │   │   ├── checkout/
    │   │   ├── orders/
    │   │   ├── payments/
    │   │   ├── customers/
    │   │   ├── inventory/
    │   │   ├── shipping/
    │   │   └── administration/
    │   │
    │   ├── shared/
    │   │   ├── components/
    │   │   ├── hooks/
    │   │   ├── types/
    │   │   ├── utils/
    │   │   └── constants/
    │   │
    │   ├── infrastructure/
    │   │   └── api/
    │   │       ├── client.ts
    │   │       ├── errors.ts
    │   │       └── interceptors.ts
    │   │
    │   ├── styles/
    │   └── main.tsx
    │
    ├── tests/
    │   ├── integration/
    │   └── e2e/
    │
    ├── index.html
    ├── package.json
    ├── tsconfig.json
    ├── vite.config.ts
    └── .env.example
```

Features should be added as the corresponding backend capabilities become available.

## 6. Communication with the Backend

The frontend communicates with the Rust API using HTTP and JSON.

Example:

```text
React Application
       |
       | HTTP / JSON
       v
Rust API
       |
       v
Application Layer
       |
       v
Domain Layer
       |
       v
Infrastructure
       |
       v
PostgreSQL
```

The frontend must not connect directly to PostgreSQL.

It must not access domain internals or bypass backend business rules.

### API Client

All HTTP communication should go through a shared API client.

Responsibilities include:

- Configure the API base URL.
- Attach authentication credentials.
- Handle HTTP errors.
- Handle expired sessions.
- Apply request timeouts.
- Support request cancellation.
- Provide consistent response handling.

Feature-specific services use this shared client.

Example:

```typescript
export async function getProducts() {
  const response = await apiClient.get("/products");
  return response.data;
}
```

## 7. API Contracts

The frontend and backend must agree on API request and response formats.

Use the following conventions:

- JSON for request and response bodies.
- Consistent naming conventions.
- Standard HTTP status codes.
- Structured error responses.
- Pagination for large collections.
- ISO 8601 timestamps.
- Explicit monetary amounts and currencies.

API contracts should be documented in the backend API documentation.

Where practical, generate TypeScript API types from the backend OpenAPI specification.

Avoid manually maintaining duplicate API models.

## 8. Authentication and Authorization

Authentication and authorization are handled by the Rust backend.

The frontend is responsible for:

- Displaying login and registration pages.
- Sending authentication requests.
- Maintaining the authenticated session.
- Displaying available features based on permissions.
- Redirecting unauthenticated users.
- Handling expired sessions.

The backend remains the authority for all access decisions.

Hiding a button or page does not provide security.

### Session Management

Prefer secure, HTTP-only cookies for browser authentication sessions.

The backend should configure:

- `HttpOnly`
- `Secure` in production
- Appropriate `SameSite` policy
- Cookie expiration
- CSRF protection where required

Avoid storing long-lived access tokens in local storage.

## 9. State Management

We distinguish between two types of frontend state.

### Server State

Data retrieved from the backend:

- Products.
- Customer information.
- Orders.
- Inventory.
- Payments.
- Shipping information.

Use TanStack Query to manage:

- API requests.
- Caching.
- Loading states.
- Error states.
- Data refresh.
- Cache invalidation.

### Client State

Temporary state managed by the frontend:

- Open menus.
- Modal visibility.
- Selected filters.
- Temporary form values.
- UI preferences.

Use React state and context where appropriate.

Do not introduce Redux unless the application develops a real need for more complex shared client state.

## 10. Error Handling

The application must provide clear feedback when something goes wrong.

Examples:

| Situation | Frontend behavior |
|---|---|
| Network unavailable | Display connection error |
| Invalid form | Display field validation errors |
| Unauthorized | Redirect to login |
| Forbidden | Display access denied |
| Product not found | Display not-found page |
| Payment failed | Display payment failure and recovery instructions |
| Server error | Display a generic error message |

Never expose internal stack traces, database errors, or sensitive backend information to users.

Business errors returned by the backend should be translated into understandable messages.

## 11. User Experience

The application should provide:

- Responsive layouts.
- Accessible forms and controls.
- Clear navigation.
- Loading indicators.
- Empty states.
- Error messages.
- Confirmation messages.
- Consistent visual components.

The customer and administration interfaces may use different layouts while sharing common UI components.

## 12. Security

Frontend security requirements include:

- Use HTTPS in production.
- Avoid storing secrets in frontend environment variables.
- Validate user input.
- Prevent unsafe HTML rendering.
- Apply appropriate Content Security Policy.
- Protect authentication sessions.
- Handle CSRF protection where applicable.
- Avoid exposing sensitive customer information.
- Never trust frontend authorization checks.

Remember that frontend validation improves user experience but does not replace backend validation.

## 13. Testing Strategy

### Unit Tests

Test individual components, hooks, and utility functions.

Examples:
- Product price formatting.
- Form validation.
- Shopping cart calculations displayed by the UI.

### Integration Tests

Test interactions between components and mocked API responses.

Examples:
- Product listing.
- Login workflow.
- Shopping cart updates.
- Checkout error handling.

### End-to-End Tests

Use Playwright to test complete user journeys.

Examples:
- Customer registration.
- Customer login.
- Product browsing.
- Add product to cart.
- Complete checkout.
- View order history.
- Administrator updates product information.

The backend remains responsible for authoritative business-rule tests.

## 14. Configuration

Frontend configuration is provided through environment variables.

Example `.env.example`:

```env
VITE_API_BASE_URL=http://localhost:8080/api/v1
```

Important:

Variables prefixed with `VITE_` are exposed to browser code after building.

Never put passwords, database credentials, API secrets, or private keys in these variables.

## 15. Development and Deployment

### Local Development

Run the Rust API and React application independently.

```text
Developer Machine
│
├── Rust API
│   └── localhost:8080
│
├── React Application
│   └── localhost:5173
│
└── PostgreSQL
    └── localhost:5432
```

Configure the API CORS policy for local development, or use a Vite development proxy.

### Production

Build the React application into static files.

Serve these files using a web server or hosting service.

The Rust API remains a separate application.

We do not need Kubernetes, Kafka, Redis, or a separate frontend infrastructure platform for the initial implementation.

## 16. Architectural Decisions

The following decisions apply to the initial implementation:

| Area | Decision |
|---|---|
| Frontend language | TypeScript |
| UI library | React |
| Build tool | Vite |
| Routing | React Router |
| Server state | TanStack Query |
| Form management | React Hook Form |
| Validation | Zod |
| Styling | Tailwind CSS |
| API communication | REST / JSON |
| Authentication | Backend-managed secure session |
| Testing | Vitest, Testing Library, Playwright |
| Deployment | Static frontend + Rust API |
| Architecture | Feature-based frontend |

## 17. Design Principles

1. Keep business rules in the Rust backend.
2. Keep frontend features independent and organized.
3. Reuse components where they provide real value.
4. Centralize API communication.
5. Generate API types from backend contracts where possible.
6. Keep authentication and authorization responsibilities clear.
7. Avoid unnecessary state-management libraries.
8. Test important customer and administrator workflows.
9. Keep the infrastructure simple.
10. Build features incrementally as backend capabilities become available.

## 18. Future Improvements

The following capabilities can be introduced when needed:

- Internationalization (i18n).
- Advanced accessibility testing.
- Frontend performance monitoring.
- Analytics.
- Product image optimization.
- Progressive Web App (PWA).
- Advanced search and filtering.
- Component documentation with Storybook.

These are not required for the initial version.
