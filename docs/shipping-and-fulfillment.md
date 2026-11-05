# Shipping and Fulfillment

## 1. Purpose

The Fulfillment context manages the process of preparing customer orders and delivering them.

It starts when an order is confirmed and ends when the order is delivered, cancelled, or marked as undeliverable.

Its responsibilities include:

- Preparing orders.
- Managing picking and packing.
- Creating shipments.
- Recording tracking information.
- Updating delivery status.
- Handling failed delivery attempts.
- Managing shipment history.

The goal is to make sure customers receive the correct products in good condition.

## 2. Responsibilities and Boundaries

Fulfillment owns shipment and delivery operations.

| Responsibility | Owner |
|---|---|
| Order details and order status | Ordering |
| Product information | Catalog |
| Available stock | Inventory |
| Shipment preparation | Fulfillment |
| Shipping address snapshot | Fulfillment |
| Tracking information | Fulfillment |
| Payment and refunds | Payment |
| Product batch information | Traceability |

Fulfillment does not manage payments or inventory quantities directly.

It communicates with Ordering and Inventory through application interfaces.

## 3. Fulfillment Workflow

The typical process is:

1. Ordering confirms an order.
2. Fulfillment receives the order reference.
3. Staff prepare the order.
4. Products are picked from inventory.
5. Products are checked and packed.
6. A shipment is created.
7. The shipment is handed to the carrier.
8. Tracking information is recorded.
9. The customer receives delivery updates.
10. The shipment is marked as delivered.

The workflow should preserve a history of each important operation.

## 4. Fulfillment Lifecycle

A fulfillment record represents the preparation and delivery process for an order.

Suggested statuses:

| Status | Meaning |
|---|---|
| Pending | Waiting for preparation |
| Picking | Products are being collected |
| Packing | Products are being packed |
| ReadyForShipment | Package is ready |
| Shipped | Package has been handed to the carrier |
| InTransit | Carrier is transporting the package |
| OutForDelivery | Package is being delivered |
| Delivered | Customer received the package |
| DeliveryFailed | Delivery attempt failed |
| Returned | Package was returned |
| Cancelled | Fulfillment was cancelled |

Not every carrier provides all these statuses.

The application should map carrier-specific statuses to its own internal statuses.

## 5. Order Preparation

When an order is confirmed, Fulfillment creates a fulfillment record.

It contains:

- Fulfillment ID.
- Order ID.
- Customer reference.
- Delivery address snapshot.
- Products and quantities.
- Preparation status.
- Creation timestamp.

Staff can view the orders waiting for preparation.

### Picking

Picking means collecting the required products from inventory.

For each product, staff should verify:

- Product identity.
- Variant.
- Quantity.
- Product condition.
- Batch information, when applicable.

If the requested product is missing or damaged, staff must report the problem before packing.

### Packing

Packing means preparing the products for shipment.

Staff should verify:

- Correct products.
- Correct quantities.
- Packaging condition.
- Product protection.
- Shipping label information.

For fragile products such as glass bottles of olive oil, appropriate packaging is important.

The system should record who completed the preparation when staff accountability is required.

## 6. Shipment Management

A shipment represents a package sent to a customer.

An order may have one or multiple shipments.

For example, if some products are unavailable at the same time, the business may decide to split the order into separate shipments.

A shipment contains:

- Shipment ID.
- Fulfillment ID.
- Order ID.
- Carrier.
- Tracking number.
- Shipping address.
- Shipment status.
- Shipped timestamp.
- Delivered timestamp.

The initial release can support one shipment per order while keeping the model flexible enough for multiple shipments later.

## 7. Carrier Integration

The application should not depend directly on a specific shipping carrier.

Use an application interface and an infrastructure adapter.

Possible future carriers include Canada Post, UPS, FedEx, or another supported provider.

### Carrier interface

```rust
pub trait ShippingProvider: Send + Sync {
    async fn create_shipment(
        &self,
        request: CreateShipmentRequest,
    ) -> Result<CreateShipmentResponse, ShippingError>;

    async fn get_tracking(
        &self,
        tracking_number: &str,
    ) -> Result<TrackingStatus, ShippingError>;

    async fn cancel_shipment(
        &self,
        tracking_number: &str,
    ) -> Result<(), ShippingError>;
}
```

The interface belongs to the application layer.

The carrier implementation belongs to infrastructure.

This allows the business to change carriers without changing its domain rules.

## 8. Shipment Tracking

Customers should be able to see the current shipment status.

Example:

```text
Order: BIO-2026-00125

Shipment status: InTransit

History:
- Order prepared
- Package packed
- Shipment created
- Package picked up by carrier
- Package arrived at distribution center
- Package in transit
```

Tracking updates can come from:

- Carrier webhooks.
- Carrier API polling.
- Manual staff updates.

For the initial release, manual updates or simple periodic polling may be sufficient.

### Tracking rules

- Validate incoming carrier events.
- Avoid processing duplicate events.
- Preserve shipment history.
- Do not move a shipment backward without a valid reason.
- Record the source of each status update.
- Keep carrier references separate from internal shipment IDs.

## 9. Delivery Management

A shipment becomes Delivered when delivery is confirmed.

Depending on carrier capabilities, confirmation may come from:

- A carrier webhook.
- A carrier tracking API.
- A staff member.
- An approved delivery confirmation process.

The application should record:

- Delivery timestamp.
- Delivery status.
- Confirmation source.
- Relevant delivery reference.

The customer can then see that the order has been delivered.

## 10. Failed Delivery

Delivery can fail for several reasons:

- Customer unavailable.
- Incorrect delivery address.
- Access problem.
- Damaged package.
- Carrier issue.

The system should record the failure reason when available.

Possible next actions include:

- Retry delivery.
- Contact the customer.
- Correct delivery information.
- Arrange pickup.
- Return the package.

A failed delivery does not automatically mean that the order is cancelled.

The business must decide what happens next.

## 11. Returns and Damaged Products

A returned shipment and a customer-requested product return are related but different processes.

Fulfillment handles the physical shipment.

Ordering and the appropriate return workflow manage the commercial decision.

Inventory determines whether returned products can be added back to sellable stock.

Traceability records relevant batch information.

For example, if a bottle arrives damaged:

1. Staff or the customer reports the damage.
2. Evidence is collected when required.
3. The return or refund process is initiated.
4. Inventory records the damaged item.
5. The product is excluded from sellable stock.

A returned item must not automatically become available inventory.

## 12. Database Structure

Fulfillment owns its database tables.

Suggested PostgreSQL structure:

```text
fulfillment/
├── fulfillments
├── fulfillment_items
├── shipments
└── shipment_events
```

### fulfillments

| Column | Description |
|---|---|
| id | Fulfillment ID |
| order_id | Related order |
| status | Fulfillment status |
| recipient_name | Delivery recipient |
| delivery_address | Address snapshot |
| created_at | Creation timestamp |
| updated_at | Last update timestamp |

### fulfillment_items

| Column | Description |
|---|---|
| id | Fulfillment item ID |
| fulfillment_id | Related fulfillment |
| product_id | Product reference |
| variant_id | Variant reference |
| quantity | Quantity to prepare |

### shipments

| Column | Description |
|---|---|
| id | Shipment ID |
| fulfillment_id | Related fulfillment |
| carrier | Carrier name |
| tracking_number | Carrier tracking reference |
| status | Shipment status |
| shipped_at | Shipment date |
| delivered_at | Delivery date |

### shipment_events

| Column | Description |
|---|---|
| id | Event ID |
| shipment_id | Related shipment |
| event_type | Shipment event |
| description | Event details |
| source | Carrier, staff, or system |
| occurred_at | Event timestamp |
| recorded_at | Database timestamp |

Use database constraints to protect relationships and prevent duplicate carrier event processing where provider event IDs are available.

## 13. Rust Project Structure

```text
src/
└── contexts/
    └── fulfillment/
        ├── domain/
        │   ├── entities/
        │   │   ├── fulfillment.rs
        │   │   └── shipment.rs
        │   ├── value_objects/
        │   │   ├── fulfillment_status.rs
        │   │   ├── shipment_status.rs
        │   │   └── tracking_number.rs
        │   ├── errors.rs
        │   └── events.rs
        │
        ├── application/
        │   ├── commands/
        │   ├── queries/
        │   ├── services/
        │   └── ports/
        │       ├── fulfillment_repository.rs
        │       └── shipping_provider.rs
        │
        └── infrastructure/
            ├── persistence/
            ├── carriers/
            └── webhooks/
```

The domain layer contains fulfillment and shipment rules.

The application layer coordinates preparation and delivery operations.

The infrastructure layer handles PostgreSQL and carrier integrations.

The domain must not depend on Axum, SQLx, or a specific carrier API.

## 14. Security and Access Control

Different staff members may have different responsibilities.

Example permissions:

| Role | Permissions |
|---|---|
| AGENT | View and prepare assigned orders |
| WAREHOUSE | Pick, pack, and update preparation |
| SUPERVISOR | Manage fulfillment and resolve issues |
| ADMIN | Manage fulfillment configuration |

These permissions are enforced through the Identity & Access context.

Important rules:

- Customers can only view their own shipments.
- Staff actions must be authorized.
- Tracking information must not expose unnecessary customer data.
- Carrier credentials must not be stored in source code.
- Sensitive delivery information must not appear in application logs.

## 15. Testing Strategy

### Unit tests

- Create a fulfillment.
- Start picking.
- Complete packing.
- Create a shipment.
- Update shipment status.
- Reject invalid state transitions.
- Mark delivery as failed.
- Confirm delivery.

### Integration tests

- Create fulfillment after order confirmation.
- Create a shipment.
- Process carrier tracking updates.
- Process duplicate carrier events.
- Handle failed delivery.
- Cancel an eligible shipment.
- Verify customer shipment access.

### Important scenario

```text
Given:
- An order is confirmed.
- Products are available for preparation.

When:
- Staff pick and pack the products.
- A shipment is created.
- The carrier confirms delivery.

Then:
- Fulfillment is completed.
- Shipment status is Delivered.
- Delivery history is preserved.
- The customer can see the delivery status.
```

## 16. Initial Release Scope

The first version should include:

- Fulfillment creation after order confirmation.
- Manual picking and packing.
- One shipment per order.
- Carrier and tracking number storage.
- Manual shipment status updates.
- Delivery confirmation.
- Shipment history.
- Staff access control.

A carrier API integration can be added when the business is ready.

Do not introduce Kafka, Redis, or a separate fulfillment microservice.

## 17. Future Improvements

Possible future features:

- Multiple shipments per order.
- Carrier API integrations.
- Automatic tracking updates.
- Shipping label generation.
- Shipping cost calculation.
- Warehouse picking optimization.
- Delivery notifications.
- Proof of delivery.
- Returns management integration.

Introduce these features based on actual business needs.

---

## Final Design Decision

Fulfillment owns the physical preparation and delivery process.

Ordering owns the commercial order, Inventory owns stock, and Fulfillment owns shipments and delivery history.

The initial implementation uses a simple workflow inside the `bio-com` modular monolith.

Carrier integrations remain behind application interfaces, allowing the system to evolve without changing its core business rules.
