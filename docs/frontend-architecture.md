# Frontend Architecture

## 1. Purpose

This document describes the frontend architecture of the `bio-com` web application.

The frontend is built with TypeScript and React. It provides the interface through which customers and administrators interact with the e-commerce platform.

The architecture should remain simple, maintainable, and easy to extend.

We use a feature-based structure to organize frontend code around business capabilities.

The Rust backend remains responsible for business rules, data integrity, authentication, and authorization.

## 2. Architecture Overview

The frontend communicates with the Rust API through REST endpoints.

```text
+--------------------------------------+
|          React Application           |
|                                      |
|  +--------------------------------+  |
|  |          Presentation          |  |
|  |                                |  |
|  | Pages, Components, Layouts     |  |
|  +--------------------------------+  |
|                 |                    |
|  +--------------------------------+  |
|  |          Application           |  |
|  |                                |  |
|  | Hooks, Workflows, UI Logic     |  |
|  +--------------------------------+  |
|                 |                    |
|  +--------------------------------+  |
|  |          API Layer             |  |
|  |                                |  |
|  | API Client, Feature Services   |  |
|  +--------------------------------+  |
+--------------------------------------+
                  |
                  | HTTP / JSON
                  v
+--------------------------------------+
|             Rust API                 |
|                                      |
|       Application / Domain           |
|                                      |
|           Infrastructure             |
|                                      |
+--------------------------------------+
                  |
                  v
              PostgreSQL
```

Each frontend layer has a clear responsibility.

## 3. Architectural Principles

The frontend follows these principles:

1. Organize code by business feature.
2. Keep components focused on presentation.
3. Keep API communication outside UI components.
4. Separate server state from local UI state.
5. Reuse shared components where appropriate.
6. Keep business rules in the Rust backend.
7. Avoid unnecessary abstractions.
8. Prefer composition over complex component inheritance.
9. Keep dependencies between features limited.
10. Test important user workflows.

## 4. Application Structure

The frontend is organized into four main areas.

### 4.1 App

Responsible for application initialization and configuration.

Includes:

- Application entry point.
- Router.
- Global providers.
- Application layouts.
- Route protection.

### 4.2 Features

Contains the functionality associated with each business capability.

Examples:

- Authentication.
- Product catalog.
- Shopping cart.
- Checkout.
- Orders.
- Payments.
- Customers.
- Inventory.
- Shipping.
- Administration.

Each feature owns its pages, components, hooks, API services, and types.

### 4.3 Shared

Contains functionality used by multiple features.

Examples:

- Buttons.
- Inputs.
- Modals.
- Tables.
- Notifications.
- Loading indicators.
- Formatting utilities.
- Common hooks.

Shared code should contain genuinely reusable functionality, not business logic belonging to a particular feature.

### 4.4 Infrastructure

Contains technical integrations.

Examples:

- HTTP client.
- API configuration.
- Authentication transport.
- Error handling.
- Request interceptors.

This area should not contain customer-facing business workflows.

## 5. Feature Organization

Each feature follows a consistent structure.

Example:

```text
features/
└── catalog/
    ├── components/
    │   ├── product-card.tsx
    │   ├── product-grid.tsx
    │   └── product-filters.tsx
    │
    ├── pages/
    │   ├── product-list-page.tsx
    │   └── product-detail-page.tsx
    │
    ├── hooks/
    │   ├── use-products.ts
    │   └── use-product.ts
    │
    ├── services/
    │   └── catalog-api.ts
    │
    ├── types/
    │   └── catalog.ts
    │
    └── index.ts
```

A feature should expose only the functionality that other features need.

For example, the checkout feature should not directly access the internal components of the catalog feature.

## 6. Component Responsibilities

Components should have clear responsibilities.

### Pages

Pages represent complete application screens.

Examples:

- Product listing page.
- Product details page.
- Shopping cart page.
- Checkout page.
- Order details page.

Pages coordinate components and data required for a screen.

### Feature Components

These components belong to a specific feature.

Examples:

- Product card.
- Cart item.
- Order summary.
- Payment status.
- Shipment tracking information.

### Shared Components

These components can be reused across features.

Examples:

- Button.
- Input.
- Dialog.
- Dropdown.
- Data table.
- Pagination.

Avoid creating shared components too early. Extract them when there is a genuine reuse requirement.

## 7. Routing

React Router manages application navigation.

Example routes:

```text
/
├── /login
├── /register
├── /products
│   └── /products/:productId
├── /cart
├── /checkout
├── /account
│   ├── /profile
│   ├── /addresses
│   └── /orders
│       └── /orders/:orderId
│
└── /admin
    ├── /dashboard
    ├── /products
    ├── /inventory
    ├── /orders
    ├── /customers
    └── /refunds
```

Routes should be organized according to customer and administrator workflows.

Protected routes improve navigation and user experience, but backend authorization remains mandatory.

## 8. API Communication

All backend communication goes through a shared HTTP client.

Feature services define API operations.

Example:

```typescript
// features/catalog/services/catalog-api.ts

import { apiClient } from "@/infrastructure/api/client";

export async function getProducts() {
  const response = await apiClient.get("/products");
  return response.data;
}

export async function getProduct(productId: string) {
  const response = await apiClient.get(
    `/products/${productId}`
  );

  return response.data;
}
```

Components should not contain direct HTTP requests.

This keeps API communication consistent and makes testing easier.

## 9. Server State Management

TanStack Query manages data retrieved from the backend.

Examples:

- Product listings.
- Customer profiles.
- Shopping carts.
- Orders.
- Payment information.
- Inventory information.

Responsibilities include:

- Fetching data.
- Caching responses.
- Tracking loading states.
- Handling request errors.
- Refreshing stale data.
- Invalidating cached data after mutations.

Example:

```typescript
import { useQuery } from "@tanstack/react-query";
import { getProducts } from "../services/catalog-api";

export function useProducts() {
  return useQuery({
    queryKey: ["products"],
    queryFn: getProducts,
  });
}
```

Query keys should be consistent and organized by feature.

## 10. Client State Management

Client state represents temporary information managed by the browser.

Examples:

- Open modal.
- Selected tab.
- Expanded menu.
- Temporary filters.
- Form interaction state.

Use React `useState` and `useContext` for simple state management.

Do not introduce Redux or another global state library without a clear requirement.

The shopping cart should have one authoritative source of truth. Persisted cart data belongs to the backend for authenticated customers.

## 11. Forms and Validation

React Hook Form manages form interactions.

Zod defines frontend validation schemas.

Example:

```typescript
import { z } from "zod";

export const loginSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

export type LoginFormData = z.infer<
  typeof loginSchema
>;
```

Frontend validation improves user experience.

The Rust backend must independently validate all submitted data and enforce business rules.

Never trust data simply because it passed frontend validation.

## 12. Authentication

Authentication is managed by the Rust backend.

The frontend provides:

- Login interface.
- Registration interface.
- Logout action.
- Session status.
- Protected navigation.
- Session expiration handling.

Prefer secure HTTP-only cookies for browser sessions.

The frontend should not store long-lived authentication credentials in local storage.

For cookie-based authentication, configure CSRF protection and appropriate cookie policies.

## 13. Authorization

The frontend can use the authenticated user's permissions to control the interface.

For example:

- Customers can access their own orders.
- Administrators can access administration screens.
- Inventory managers can access inventory operations.

The frontend may hide unavailable actions or navigation items.

However, every protected API operation must independently verify authorization on the backend.

The frontend is never the security boundary.

## 14. Error Handling

API errors should follow a consistent handling strategy.

The shared API layer handles technical errors.

Feature components handle errors that require specific user feedback.

Examples:

| Error | Expected behavior |
|---|---|
| Network failure | Display connection error |
| Unauthorized | Handle expired session |
| Forbidden | Display access denied |
| Validation failure | Display field errors |
| Resource not found | Display not-found state |
| Payment failure | Display payment recovery instructions |
| Internal server error | Display generic error |

Do not expose backend implementation details to the browser.

## 15. Frontend Security

The application must follow basic browser security practices.

- Use HTTPS in production.
- Never expose private API credentials.
- Use secure session cookies.
- Apply CSRF protection where applicable.
- Avoid unsafe HTML rendering.
- Sanitize untrusted content where necessary.
- Apply an appropriate Content Security Policy.
- Avoid exposing sensitive customer data.
- Never trust client-side authorization.
- Keep dependencies updated.

## 16. Performance

The frontend should remain responsive as the application grows.

Initial performance practices:

- Lazy-load routes.
- Avoid unnecessary component re-renders.
- Use server-side pagination for large collections.
- Optimize product images.
- Cache API responses appropriately.
- Avoid loading unnecessary data.
- Split application bundles when useful.

Performance optimization should be guided by actual measurements.

## 17. Testing

### Unit Tests

Test individual components, hooks, and utilities.

### Integration Tests

Test interactions between components and API services.

### End-to-End Tests

Use Playwright to test complete customer and administrator workflows.

Important scenarios include:

- Customer registration and login.
- Product browsing.
- Cart management.
- Checkout.
- Order history.
- Payment failure handling.
- Administration product management.
- Inventory updates.

Tests should verify user-visible behavior rather than implementation details.

## 18. Dependency Management

Keep dependencies limited to the technologies required by the application.

The initial stack is:

| Concern | Library |
|---|---|
| UI | React |
| Language | TypeScript |
| Build | Vite |
| Routing | React Router |
| API state | TanStack Query |
| Forms | React Hook Form |
| Validation | Zod |
| Styling | Tailwind CSS |
| Unit testing | Vitest |
| Component testing | React Testing Library |
| E2E testing | Playwright |

Avoid adding additional libraries without a clear technical reason.

## 19. Architectural Boundaries

The frontend must not:

- Connect directly to PostgreSQL.
- Implement authoritative business rules.
- Calculate authoritative order totals.
- Decide whether a refund is eligible.
- Decide whether an order can be cancelled.
- Authorize access to protected resources.
- Communicate directly with internal database infrastructure.

These responsibilities belong to the Rust backend.

The frontend displays the results of backend decisions and provides the necessary user interactions.

## 20. Architectural Decisions

| Area | Decision |
|---|---|
| Architecture | Feature-based |
| UI | React |
| Language | TypeScript |
| Routing | React Router |
| Server state | TanStack Query |
| Client state | React state and context |
| Forms | React Hook Form |
| Validation | Zod |
| API | REST / JSON |
| Authentication | Backend-managed sessions |
| Styling | Tailwind CSS |
| Testing | Vitest and Playwright |
| Backend | Rust |
| Database access | Through Rust API only |

## 21. Design Principles

1. Keep frontend responsibilities separate from backend responsibilities.
2. Organize features around user and business capabilities.
3. Keep components small and focused.
4. Centralize API communication.
5. Use TypeScript for reliable contracts.
6. Separate server state from client state.
7. Keep authentication and authorization secure.
8. Reuse components where appropriate.
9. Test complete user journeys.
10. Avoid unnecessary frontend complexity.

## 22. Future Improvements

Potential improvements include:

- Storybook for component documentation.
- Internationalization.
- Accessibility automation.
- Frontend observability.
- Advanced performance monitoring.
- Progressive Web App support.

These improvements should be introduced when justified by actual requirements.
