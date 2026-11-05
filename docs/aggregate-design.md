# bio-com — Aggregate Design

## 1. Purpose

This document defines the aggregate boundaries of bio-com.

An aggregate is a group of related domain objects that must follow business rules together.

Every aggregate has one entry point called the **Aggregate Root**.

Other parts of the application must use the aggregate root to change its internal state.

For example:

- An Order contains OrderItems.
- A StockItem manages stock quantities.
- A Payment manages payment status.
- A Product manages its variants.

We use aggregates to protect business consistency and prevent invalid changes.

## 2. Aggregate Design Principles

We will follow these rules:

1. Every aggregate has one root.
2. External code modifies an aggregate through its root.
3. Aggregates own their business rules.
4. Aggregates should remain small.
5. Aggregates should not directly depend on other aggregates.
6. Other aggregates are referenced through identifiers.
7. A database transaction should normally update one aggregate at a time.
8. Cross-aggregate workflows must handle failures explicitly.

We should not create large aggregates simply because objects are related.

For example, an Order should not contain the complete Customer, Product, Inventory, and Payment objects.

It should reference the necessary identifiers and preserve the information needed for the purchase.

## 3. Aggregate Overview

| Context | Aggregate Root | Internal Entities |
|---|---|---|
| Identity & Access | User | None initially |
| Identity & Access | Role | None initially |
| Customer | Customer | CustomerAddress |
| Catalog | Product | ProductVariant |
| Pricing | PriceList | Price entries |
| Pricing | Promotion | Promotion rules |
| Cart | Cart | CartItem |
| Inventory | StockItem | None initially |
| Inventory | StockReservation | None initially |
| Ordering | Order | OrderItem |
| Payment | Payment | PaymentAttempt |
| Payment | Refund | Refund records |
| Fulfillment | Fulfillment | Shipment |
| Traceability | ProductBatch | None initially |

Some of these aggregates may be refined as implementation requirements become clearer.

## 4. Catalog Aggregates

### 4.1 Product Aggregate

**Root:** Product

The Product aggregate manages product information and its variants.

Example:

Organic Olive Oil

- 250 ml
- 500 ml
- 1 litre

Responsibilities:

- Create products.
- Update product information.
- Add variants.
- Update variants.
- Publish products.
- Unpublish products.

Invariants:

- Product name cannot be empty.
- A published product must have at least one valid variant.
- Each variant must have a valid SKU.
- A variant must belong to its parent product.

External reference:

- CategoryId

The Product aggregate does not manage prices or stock.

## 5. Pricing Aggregates

### 5.1 PriceList Aggregate

**Root:** PriceList

Manages prices for products and variants.

Responsibilities:

- Define prices.
- Update prices.
- Activate price lists.
- Deactivate price lists.

Invariants:

- Prices cannot be negative.
- Currency must be defined.
- Only valid price lists can be used for calculations.
- Price changes must preserve historical order prices.

### 5.2 Promotion Aggregate

**Root:** Promotion

Manages promotional rules.

Responsibilities:

- Create promotions.
- Configure discounts.
- Activate promotions.
- Expire promotions.
- Validate promotion eligibility.

Invariants:

- Discount must respect configured limits.
- Promotion dates must be valid.
- Expired promotions cannot be applied.
- Promotion conditions must be satisfied.

## 6. Cart Aggregate

### 6.1 Cart Aggregate

**Root:** Cart

Contains:

- CartItem

Responsibilities:

- Add items.
- Remove items.
- Change quantities.
- Clear cart.
- Calculate cart summary.
- Prepare checkout.

Invariants:

- Cart item quantity must be greater than zero.
- An empty cart cannot be checked out.
- A cart cannot contain invalid variants.
- Cart totals must be recalculated when quantities change.

External references:

- CustomerId
- ProductId
- VariantId

The Cart aggregate does not reserve stock permanently.

Stock must be validated during checkout.

## 7. Inventory Aggregates

Inventory requires special attention because multiple customers may purchase the same product simultaneously.

### 7.1 StockItem Aggregate

**Root:** StockItem

Represents the stock of one product variant at one stock location.

Responsibilities:

- Receive stock.
- Adjust stock.
- Deduct confirmed stock.
- Calculate available quantity.
- Track reserved quantity.

Invariants:

- Physical quantity cannot be negative.
- Reserved quantity cannot exceed physical quantity.
- Available quantity cannot be negative.

Formula:

`Available = Physical - Reserved`

Example:

| Quantity Type | Amount |
|---|---:|
| Physical | 100 |
| Reserved | 20 |
| Available | 80 |

### 7.2 StockReservation Aggregate

**Root:** StockReservation

Represents a temporary stock reservation for an order.

Responsibilities:

- Create reservation.
- Confirm reservation.
- Release reservation.
- Expire reservation.

Invariants:

- Reservation quantity must be positive.
- A reservation must reference an order.
- A reservation cannot be confirmed twice.
- A released reservation cannot be confirmed.
- An expired reservation cannot be confirmed.

Reservation lifecycle:

`Active → Confirmed`

`Active → Released`

`Active → Expired`

Stock reservation and stock deduction must be coordinated carefully.

### 7.3 Concurrency Protection

Imagine two customers trying to purchase the last bottle of olive oil.

Both customers see one available bottle.

Only one reservation should succeed.

We will use PostgreSQL transactions and row-level locking or atomic conditional updates to prevent overselling.

The application must not rely only on checking stock in Rust before writing to the database.

## 8. Ordering Aggregate

### 8.1 Order Aggregate

**Root:** Order

Contains:

- OrderItem

Responsibilities:

- Create order.
- Add order items during creation.
- Calculate totals.
- Confirm order.
- Cancel order.
- Update allowed order states.

Each OrderItem stores a purchase snapshot:

- ProductId.
- VariantId.
- SKU.
- Product name.
- Variant description.
- Unit price.
- Quantity.
- Line total.

Invariants:

- Order must contain at least one item.
- Quantities must be positive.
- Monetary amounts must be valid.
- Order total must match its line items and discounts.
- Confirmed orders cannot be modified freely.
- Delivered orders cannot be cancelled through the normal cancellation operation.

### 8.2 Order Lifecycle

Initial lifecycle:

`Pending → Confirmed → Processing → Shipped → Delivered`

Cancellation:

`Pending → Cancelled`

`Confirmed → Cancelled` only when the cancellation policy allows it.

The exact transitions will be defined in the order state machine.

### 8.3 Order Snapshot

An order must preserve the information agreed upon at purchase time.

For example:

A customer purchases olive oil for $15.

The next day, the price increases to $17.

The existing order must still show $15.

This is why OrderItem stores a price snapshot rather than depending on the current Pricing context.

## 9. Payment Aggregates

### 9.1 Payment Aggregate

**Root:** Payment

Contains:

- PaymentAttempt

Responsibilities:

- Initiate payment.
- Record payment attempts.
- Mark payment as successful.
- Mark payment as failed.
- Track provider references.

Invariants:

- Payment must reference an OrderId.
- Payment amount must be positive.
- A successful payment cannot become pending again.
- Duplicate provider notifications must not duplicate financial operations.
- Every payment attempt must be traceable.

### 9.2 Refund Aggregate

**Root:** Refund

Responsibilities:

- Request refund.
- Approve refund when required.
- Process refund.
- Record refund failure.

Invariants:

- Refund amount must be positive.
- Refund cannot exceed the refundable amount.
- Duplicate refund processing must be prevented.
- Refund status must follow valid transitions.

A payment can have multiple refund operations as long as their combined amount does not exceed the refundable amount.

## 10. Fulfillment Aggregate

### 10.1 Fulfillment Aggregate

**Root:** Fulfillment

Contains:

- Shipment

Responsibilities:

- Start fulfillment.
- Prepare products.
- Pack order.
- Create shipment.
- Update delivery information.
- Complete fulfillment.

Invariants:

- Fulfillment must reference an OrderId.
- Shipment must have valid delivery information.
- An order cannot be shipped before it is ready.
- Shipment updates must follow valid transitions.

## 11. Traceability Aggregate

### 11.1 ProductBatch Aggregate

**Root:** ProductBatch

Responsibilities:

- Register production batch.
- Update batch details.
- Record origin.
- Record production date.
- Record expiration date.
- Track batch status.

Invariants:

- Batch number must be unique within its scope.
- Production date must be valid.
- Expiration date must not precede production date.
- Batch must reference a valid product or variant.

Inventory can reference the BatchId when receiving or managing stock.

## 12. Cross-Aggregate Workflows

Some business operations involve multiple aggregates.

These operations must not create one giant aggregate.

Instead, we coordinate them through application services.

### Example: Customer Checkout

1. Validate cart.
2. Retrieve current prices.
3. Validate product availability.
4. Reserve stock.
5. Create order.
6. Initiate payment.
7. Confirm payment.
8. Confirm order.
9. Confirm stock deduction.
10. Start fulfillment when appropriate.

Each aggregate protects its own rules.

The application layer coordinates the workflow.

If something fails, the application must handle the failure explicitly.

For example:

- Payment fails: release the reservation and mark the order appropriately.
- Order creation fails: release the reservation.
- Payment succeeds but confirmation fails: preserve the payment result and initiate recovery rather than blindly charging again.

This workflow will be refined in the checkout and order lifecycle documents.

## 13. Transaction Boundaries

Our initial transaction strategy:

| Operation | Transaction Boundary |
|---|---|
| Create product | Product aggregate |
| Update price list | PriceList aggregate |
| Add cart item | Cart aggregate |
| Reserve stock | StockItem and reservation operation |
| Create order | Order aggregate |
| Record payment result | Payment aggregate |
| Process refund result | Refund aggregate |
| Create shipment | Fulfillment aggregate |

For operations involving multiple aggregates, we will use application-level orchestration and explicit recovery rules.

External payment API calls must never be held inside long-running database transactions.

## 14. Domain Events

Aggregates may raise domain events after successful business operations.

Examples:

| Aggregate | Event |
|---|---|
| Product | ProductPublished |
| Cart | CartCheckedOut |
| StockReservation | StockReserved |
| StockReservation | StockReleased |
| Order | OrderCreated |
| Order | OrderConfirmed |
| Order | OrderCancelled |
| Payment | PaymentSucceeded |
| Refund | RefundProcessed |
| Fulfillment | ShipmentCreated |
| Fulfillment | OrderDelivered |

Events will initially be handled in-process.

For reliable asynchronous processing across application restarts, we can introduce a PostgreSQL-backed transactional outbox when the relevant workflows require it.

## 15. Rust Implementation Guidelines

The domain layer will use plain Rust types.

Example:

```rust
pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    items: Vec<OrderItem>,
    status: OrderStatus,
    total: Money,
}
```

The fields are private to protect the aggregate.

Changes happen through methods:

```rust
impl Order {
    pub fn confirm(&mut self) -> Result<(), OrderError> {
        if self.items.is_empty() {
            return Err(OrderError::EmptyOrder);
        }

        if self.status != OrderStatus::Pending {
            return Err(OrderError::InvalidStatus);
        }

        self.status = OrderStatus::Confirmed;

        Ok(())
    }
}
```

This is a simplified illustration. Production implementation will also handle payment, inventory, and checkout workflow requirements outside the Order aggregate.

The domain layer must not import SQLx or Axum.

## 16. Final Design Decisions

| Decision | Approach |
|---|---|
| Aggregate roots | Small and business-focused |
| Entity modification | Through aggregate root |
| Cross-aggregate references | Identifiers |
| Order items | Inside Order |
| Product variants | Inside Product |
| Stock reservations | Explicit lifecycle |
| Payment | Separate from Order |
| Refund | Separate aggregate |
| Money | Precise monetary representation |
| Concurrency | PostgreSQL transaction protection |
| Domain events | In-process initially |
| External workflows | Application orchestration |
| Infrastructure | Lightweight modular monolith |

## 17. Summary

The aggregate design defines where business consistency is protected.

The most important boundaries are:

- Product protects catalog rules.
- Cart protects shopping selections.
- StockItem protects inventory quantities.
- StockReservation protects reservation lifecycle.
- Order protects purchase information.
- Payment protects payment state.
- Refund protects refund operations.
- Fulfillment protects delivery operations.
- ProductBatch protects traceability information.

These boundaries will guide the Rust domain implementation and PostgreSQL persistence design.

**Next step:** Define the detailed business workflows and state transitions, starting with checkout, ordering, inventory reservations, and payment.
