# bio-com — Domain Model

## 1. Purpose

This document describes the main building blocks of the bio-com domain model.

We use Domain-Driven Design (DDD) to represent business concepts in code.

The domain model contains:

- **Entities:** Objects with a unique identity and lifecycle.
- **Value Objects:** Objects defined by their values rather than identity.
- **Aggregates:** Groups of related objects that must follow business rules together.
- **Aggregate Roots:** The main entry points for modifying an aggregate.
- **Domain Services:** Business operations that do not naturally belong to one entity.
- **Domain Events:** Important business facts that have happened.

The goal is to keep business rules inside the domain rather than spreading them across API handlers, database queries, and application services.

## 2. Domain Modeling Principles

We will follow these principles:

### 2.1 Protect business rules

Business rules must be enforced by the domain model.

For example, an order cannot be confirmed if it has no items.

### 2.2 Use Value Objects

Important business concepts should have their own types.

For example, use `Money` instead of passing a floating-point number everywhere.

### 2.3 Keep aggregates small

An aggregate should contain only the objects needed to maintain its consistency.

We should not load an entire customer history whenever we update an order.

### 2.4 Control modifications through aggregate roots

Other parts of the application must use the aggregate root to modify its internal objects.

For example, an `OrderItem` should be modified through `Order`, not independently.

### 2.5 Keep business logic independent of infrastructure

The domain model must not depend on:

- Axum.
- SQLx.
- PostgreSQL.
- HTTP requests.
- External payment providers.

The domain should work independently of these technologies.

## 3. Identity & Access Context

### 3.1 Main Aggregate Roots

| Aggregate | Responsibility |
|---|---|
| User | Manage user account and status |
| Role | Manage role information and assigned permissions |

### 3.2 Entities

- User
- Role

### 3.3 Value Objects

- UserId
- RoleId
- Email
- PasswordHash
- Permission
- AccountStatus

### 3.4 Business Rules

- Every user must have a unique identity.
- Email addresses must follow the defined validation rules.
- Passwords must be securely hashed.
- Inactive users cannot authenticate.
- A role can contain multiple permissions.
- Authorization must be checked before protected operations.

## 4. Customer Context

### 4.1 Aggregate Root

**Customer**

The Customer aggregate manages customer business information.

### 4.2 Entities

- Customer

### 4.3 Value Objects

- CustomerId
- CustomerName
- Email
- PhoneNumber
- Address
- CountryCode

### 4.4 Business Rules

- A customer must have valid identifying information.
- A customer can have multiple delivery addresses.
- Address changes must not silently modify historical order addresses.
- Customer records must have a clear lifecycle.

The Order aggregate will keep a snapshot of the delivery address used at checkout.

## 5. Catalog Context

### 5.1 Aggregate Root

**Product**

The Product aggregate represents an item offered by the business.

### 5.2 Entities

- Product
- ProductVariant

### 5.3 Value Objects

- ProductId
- VariantId
- ProductName
- ProductDescription
- SKU
- CategoryId
- ProductStatus
- ProductAttribute

### 5.4 Business Rules

- A product must have a valid name.
- A product must have at least one valid variant before publication.
- Each variant must have a unique SKU within the agreed business scope.
- An unpublished product cannot be purchased.
- Product variants belong to their parent product.
- Product information must be validated before publication.

Example:

Product: Organic Olive Oil

Variants:

- 250 ml — SKU: OLIVE-250
- 500 ml — SKU: OLIVE-500
- 1 litre — SKU: OLIVE-1000

## 6. Pricing Context

### 6.1 Aggregate Roots

| Aggregate | Responsibility |
|---|---|
| PriceList | Manage product and variant prices |
| Promotion | Manage promotional rules |

### 6.2 Entities

- PriceList
- Promotion

### 6.3 Value Objects

- PriceListId
- PromotionId
- Money
- Currency
- DiscountPercentage
- ValidityPeriod
- Price

### 6.4 Business Rules

- Prices must not be negative.
- Currency must be explicitly defined.
- Discounts must respect configured limits.
- Promotions must have valid start and end dates.
- Expired promotions cannot be applied.
- The final price must never be negative.

Pricing calculations must use precise decimal or integer-based monetary representations, never floating-point arithmetic.

## 7. Cart Context

### 7.1 Aggregate Root

**Cart**

The Cart aggregate manages a customer's selected products.

### 7.2 Entities

- Cart
- CartItem

### 7.3 Value Objects

- CartId
- CartItemId
- ProductId
- VariantId
- Quantity
- CartStatus

### 7.4 Business Rules

- A cart belongs to a customer or an identified guest session.
- A cart item must have a valid quantity.
- The same variant should not appear as multiple independent lines unless business requirements explicitly allow it.
- Empty carts cannot be checked out.
- Cart prices must be revalidated during checkout.
- Cart availability must be checked before order creation.

A cart is not a financial commitment. Final prices and stock must be validated when creating an order.

## 8. Inventory Context

### 8.1 Aggregate Roots

| Aggregate | Responsibility |
|---|---|
| StockItem | Manage stock for a product variant and location |
| StockReservation | Manage reserved quantities |

### 8.2 Entities

- StockItem
- StockReservation
- InventoryMovement

### 8.3 Value Objects

- StockItemId
- ReservationId
- ProductId
- VariantId
- WarehouseId
- Quantity
- StockStatus
- MovementType

### 8.4 Business Rules

- Physical stock cannot be negative.
- Available stock cannot be negative.
- Reserved stock cannot exceed physical stock.
- A reservation must have an expiration or defined lifecycle.
- Confirmed reservations must not be deducted twice.
- Released reservations cannot be confirmed afterward.
- Every stock adjustment must be recorded.

Example:

Physical quantity: 50

Reserved quantity: 10

Available quantity: 40

When an order is confirmed, the reservation is converted into a stock deduction according to the inventory workflow.

## 9. Ordering Context

### 9.1 Aggregate Root

**Order**

The Order aggregate manages the customer's purchase.

### 9.2 Entities

- Order
- OrderItem

### 9.3 Value Objects

- OrderId
- OrderItemId
- CustomerId
- ProductId
- VariantId
- SKU
- Money
- Quantity
- ShippingAddress
- OrderStatus
- OrderReference

### 9.4 Business Rules

- An order must contain at least one item.
- An order must belong to a customer.
- Each item must have a valid quantity.
- The order total must match its item and discount calculations.
- Historical product and price information must be preserved.
- An order cannot be confirmed before the required checkout validations succeed.
- A delivered order cannot be cancelled through the normal cancellation workflow.
- Order status transitions must follow defined business rules.

Example lifecycle:

`Pending → Confirmed → Processing → Shipped → Delivered`

Cancellation and refund handling must follow separate, explicit rules.

## 10. Payment Context

### 10.1 Aggregate Roots

| Aggregate | Responsibility |
|---|---|
| Payment | Manage payment lifecycle |
| Refund | Manage refund lifecycle |

### 10.2 Entities

- Payment
- PaymentAttempt
- Refund

### 10.3 Value Objects

- PaymentId
- PaymentAttemptId
- RefundId
- OrderId
- Money
- PaymentMethod
- PaymentStatus
- RefundStatus
- ProviderReference

### 10.4 Business Rules

- A payment must reference an order.
- Payment amounts must be positive.
- A successful payment must not be processed twice.
- Provider callbacks must be validated.
- Refund amounts cannot exceed the refundable amount.
- Payment and refund operations must be traceable.
- Payment failures must not be treated as successful payments.

Payment operations must be idempotent to protect against duplicate requests and provider callbacks.

## 11. Fulfillment Context

### 11.1 Aggregate Root

**Fulfillment**

The Fulfillment aggregate manages the preparation and delivery of an order.

### 11.2 Entities

- Fulfillment
- Shipment

### 11.3 Value Objects

- FulfillmentId
- ShipmentId
- OrderId
- TrackingNumber
- Carrier
- FulfillmentStatus
- DeliveryStatus

### 11.4 Business Rules

- A fulfillment must reference an existing order.
- Products must be prepared before shipment.
- A shipment must have valid delivery information.
- Delivery status must follow valid transitions.
- A shipment must not be created twice accidentally for the same fulfillment operation.

## 12. Traceability Context

### 12.1 Aggregate Root

**ProductBatch**

The ProductBatch aggregate manages information about a production batch.

### 12.2 Entities

- ProductBatch

### 12.3 Value Objects

- BatchId
- BatchNumber
- ProductId
- VariantId
- Origin
- ProductionDate
- ExpirationDate
- SupplierReference

### 12.4 Business Rules

- Every batch must have a unique batch number within its defined scope.
- A batch must reference a valid product or variant.
- Expiration dates must follow production-date rules.
- Batch origin information must be recorded when required.
- Expired batches must not be sold when product policy prohibits it.
- Batch history must be traceable.

## 13. Aggregate Relationships

The following diagram shows the main aggregate roots.

```mermaid
flowchart TD
    subgraph Catalog
        Product[Product]
    end

    subgraph Pricing
        PriceList[PriceList]
        Promotion[Promotion]
    end

    subgraph Cart
        Cart[Cart]
    end

    subgraph Inventory
        StockItem[StockItem]
        Reservation[StockReservation]
    end

    subgraph Ordering
        Order[Order]
    end

    subgraph Payment
        Payment[Payment]
        Refund[Refund]
    end

    subgraph Fulfillment
        Fulfillment[Fulfillment]
    end

    subgraph Traceability
        Batch[ProductBatch]
    end

    Product --> Cart
    PriceList --> Cart
    Promotion --> Cart

    Cart --> Order
    Product --> StockItem
    StockItem --> Reservation
    Reservation --> Order

    Order --> Payment
    Payment --> Refund
    Order --> Fulfillment

    Product --> Batch
    Batch --> StockItem
```

The diagram shows conceptual relationships. It does not mean that aggregates share the same database transaction.

## 14. Domain Events

Domain events represent important business facts that have already happened.

Initial events may include:

| Context | Event | Meaning |
|---|---|---|
| Identity & Access | UserRegistered | A user account was created |
| Catalog | ProductPublished | A product became available |
| Pricing | PromotionActivated | A promotion became active |
| Cart | CartCheckedOut | Checkout was requested successfully |
| Inventory | StockReserved | Stock was reserved |
| Inventory | StockReservationReleased | Reserved stock was released |
| Ordering | OrderCreated | An order was created |
| Ordering | OrderConfirmed | An order was confirmed |
| Ordering | OrderCancelled | An order was cancelled |
| Payment | PaymentSucceeded | A payment succeeded |
| Payment | RefundProcessed | A refund was processed |
| Fulfillment | ShipmentCreated | A shipment was created |
| Fulfillment | OrderDelivered | An order was delivered |

Events should be raised by the domain when business operations succeed.

For the initial modular monolith, events can be handled in-process. We do not need an external message broker.

## 15. Aggregate Consistency Rules

An aggregate protects its own business consistency.

For example:

- Order protects order items and order totals.
- Product protects its variants and publication rules.
- Cart protects cart items and quantities.
- StockItem protects physical, reserved, and available quantities.
- Payment protects payment state transitions.

When an operation requires changes across multiple aggregates, we must decide where the transaction boundary belongs.

We should avoid treating the entire e-commerce workflow as one large database transaction.

For example, payment processing involves an external provider. It cannot safely be included in a normal PostgreSQL transaction.

Instead, we will use explicit workflow states, idempotency, and failure recovery.

## 16. Modeling Decisions to Confirm

Some business decisions require further refinement before implementation.

| Topic | Initial direction |
|---|---|
| Product variants | Variants belong to Product |
| SKU | Unique within the catalog |
| Money | Decimal or integer-based representation |
| Currency | Explicit currency per monetary amount |
| Cart | Temporary customer shopping state |
| Order | Preserves historical product and price snapshots |
| Inventory | Stock tracked by variant and warehouse |
| Reservation | Separate lifecycle from physical stock |
| Payment | Separate from Order |
| Refund | Separate lifecycle from Payment |
| Traceability | Batch-level product tracking |
| Domain events | In-process initially |

These decisions will be refined through the detailed context design.

## 17. Summary

The bio-com domain model is built around small aggregates with clear business responsibilities.

The most important aggregates in the initial design are:

- Product
- Cart
- StockItem
- StockReservation
- Order
- Payment
- Refund
- Fulfillment
- ProductBatch

Each aggregate protects its own rules and exposes operations through its root.

This model will guide our Rust implementation, database design, and application use cases.

**Next step:** Define the detailed aggregate boundaries and business invariants before implementation.
