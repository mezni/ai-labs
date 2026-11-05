# Payment Management

## 1. Purpose

The Payment context handles the financial side of customer orders.

It is responsible for:

- Creating and tracking payments.
- Communicating with external payment providers.
- Recording payment attempts.
- Handling payment confirmations and failures.
- Processing full and partial refunds.
- Keeping a reliable history of financial operations.

The goal is to make payments secure, reliable, and easy to maintain.

## 2. Responsibilities and Boundaries

The Payment context owns payment-related information.

| Responsibility | Owner |
|---|---|
| Order details and order status | Ordering |
| Payment status and payment attempts | Payment |
| Refund records and refund status | Payment |
| Available stock and reservations | Inventory |
| Product information | Catalog |
| Payment provider communication | Payment infrastructure adapter |

Payment does not manage orders or inventory directly. It communicates with these contexts through application interfaces.

**Important:** An order being confirmed does not automatically mean that its payment has been captured. Ordering and Payment maintain separate states.

## 3. Payment Concepts

### Payment

A payment represents the financial operation associated with an order.

It contains:

- Payment ID.
- Order ID.
- Amount.
- Currency.
- Payment status.
- Payment provider.
- Provider reference.
- Creation and update timestamps.

One order can have multiple payment attempts, but the system must prevent accidental duplicate charges.

### Payment Attempt

A payment attempt represents one interaction with a payment provider.

For example:

1. A customer starts checkout.
2. The system creates a payment attempt.
3. The provider processes the payment.
4. The provider returns a result.
5. The system updates the payment status.

If the first attempt fails, the customer may try again.

### Refund

A refund returns money to the customer after a successful payment.

The system supports:

- Full refunds.
- Partial refunds.
- Multiple partial refunds, within the original refundable amount.

A refund must always reference its original payment.

## 4. Payment Lifecycle

The initial payment workflow uses a simple payment-intent or hosted-checkout approach.

The exact provider behavior will be handled by an adapter because providers do not all use identical payment states.

### Payment statuses

| Status | Meaning |
|---|---|
| Pending | Payment has been created but not completed |
| RequiresAction | Customer must complete an additional step |
| Processing | Provider is still processing the payment |
| Succeeded | Provider confirmed successful payment |
| Failed | Provider confirmed payment failure |
| Cancelled | Payment was cancelled |

A provider timeout must not automatically change a payment to Failed. The result may be unknown.

The application must check the provider or wait for a verified webhook before deciding what happened.

### Typical workflow

1. Customer confirms the order.
2. Ordering creates an order awaiting payment.
3. Payment creates a payment record.
4. Payment requests a checkout session or payment intent from the provider.
5. Customer completes the payment.
6. Provider confirms the result.
7. Payment updates its status.
8. The application workflow notifies Ordering.
9. Ordering confirms the order after successful payment.

The customer should never be charged again simply because the application did not receive the first response.

## 5. Refund Lifecycle

Refunds have their own lifecycle.

| Status | Meaning |
|---|---|
| Requested | Refund has been requested |
| Processing | Provider is processing the refund |
| Succeeded | Refund was confirmed |
| Failed | Provider confirmed that the refund failed |

### Refund rules

- A refund can only reference a successful payment.
- The refund amount must be greater than zero.
- The total refunded amount cannot exceed the original captured amount.
- Partial refunds are allowed.
- A refund request must be idempotent.
- Failed refunds must remain visible for investigation.

Example:

An order was paid for $120.

The customer receives a partial refund of $30.

The remaining refundable amount is $90.

A later refund of $100 must be rejected.

## 6. Payment Provider Integration

The domain must not depend directly on a specific payment provider.

Instead, the application defines an interface and infrastructure implements it.

Possible future providers include Stripe, Adyen, or another provider that supports the required payment operations.

### Provider interface

```rust
pub trait PaymentProvider: Send + Sync {
    async fn create_payment(
        &self,
        request: CreatePaymentRequest,
    ) -> Result<CreatePaymentResponse, PaymentProviderError>;

    async fn get_payment(
        &self,
        provider_reference: &str,
    ) -> Result<ProviderPaymentStatus, PaymentProviderError>;

    async fn refund(
        &self,
        request: RefundPaymentRequest,
    ) -> Result<RefundPaymentResponse, PaymentProviderError>;
}
```

The interface belongs to the application layer.

The provider implementation belongs to the infrastructure layer.

This keeps business rules independent of external APIs.

## 7. Webhook Handling

Payment providers often send webhooks to notify the application about payment results.

For example:

- Payment succeeded.
- Payment failed.
- Refund succeeded.
- Refund failed.

Webhooks are important because a customer may close the browser before the application receives the payment result.

### Webhook processing

1. Receive the webhook.
2. Verify its signature.
3. Validate the event structure.
4. Check whether the event was already processed.
5. Store the event for reliable processing.
6. Update the corresponding payment or refund.
7. Notify the relevant application workflow.
8. Return the appropriate HTTP response.

### Important rules

- Never trust a webhook without verifying its signature.
- Never process the same event twice.
- Store provider event IDs.
- Reject or safely ignore invalid events.
- Do not assume events arrive in order.
- Do not expose payment secrets in logs.

A webhook inbox table can provide reliable event tracking without requiring Kafka or another message broker.

## 8. Idempotency

Idempotency prevents the same operation from being executed more than once.

This is especially important for payments and refunds.

For example, a customer clicks the Pay button twice because the first request appears slow.

The system should not create two charges.

Each payment or refund operation should have an idempotency key.

The key must be stored and checked before submitting a new provider operation.

Provider-side idempotency should also be used whenever supported.

## 9. Failure Handling and Reconciliation

External payment operations can fail in several ways:

- The provider rejects the payment.
- The customer abandons checkout.
- The network connection times out.
- The provider processes the payment but the application does not receive the response.
- A webhook arrives more than once.
- A webhook arrives before another related event.

The system must distinguish between a confirmed failure and an unknown result.

### Example: Payment timeout

1. The application sends a payment request.
2. The provider processes it successfully.
3. The network connection times out.
4. The application does not know whether the payment succeeded.

The correct response is to mark the operation as unresolved or keep it in a processing state.

The application should query the provider using the existing reference or wait for a verified webhook.

It must not immediately create another charge.

### Reconciliation

Reconciliation compares local payment records with provider records.

For the initial release, this can be performed through:

- An administrative operation.
- A scheduled application task.
- A manual investigation process.

A separate infrastructure platform is not required.

## 10. Security Requirements

Payment information must be handled carefully.

The application must:

- Never store full card numbers.
- Never store CVV codes.
- Prefer hosted checkout or provider tokenization.
- Keep provider credentials outside source code.
- Use HTTPS for payment and webhook communication.
- Verify webhook signatures.
- Restrict access to refund operations.
- Record who requested or approved a refund.
- Avoid logging sensitive payment information.

Payment authorization must be enforced through the Identity & Access context.

For example, only authorized staff should be able to initiate or approve certain refunds.

## 11. Database Structure

The Payment context owns its database tables.

Initial PostgreSQL structure:

```text
payment/
├── payments
├── payment_attempts
├── refunds
└── webhook_events
```

### payments

| Column | Description |
|---|---|
| id | Payment ID |
| order_id | Related order ID |
| amount | Original payment amount |
| currency | Payment currency |
| status | Current payment status |
| provider | Payment provider name |
| provider_reference | Provider payment reference |
| created_at | Creation timestamp |
| updated_at | Last update timestamp |

### payment_attempts

| Column | Description |
|---|---|
| id | Attempt ID |
| payment_id | Related payment |
| attempt_number | Attempt sequence |
| status | Attempt status |
| provider_reference | Provider reference |
| failure_code | Failure reason, when available |
| created_at | Creation timestamp |

### refunds

| Column | Description |
|---|---|
| id | Refund ID |
| payment_id | Original payment |
| amount | Refund amount |
| currency | Refund currency |
| status | Refund status |
| provider_reference | Provider refund reference |
| reason | Refund reason |
| created_at | Creation timestamp |

### webhook_events

| Column | Description |
|---|---|
| id | Internal event ID |
| provider_event_id | Unique provider event ID |
| event_type | Provider event type |
| status | Processing status |
| received_at | Reception timestamp |
| processed_at | Processing timestamp |

The database must enforce uniqueness for provider event IDs and appropriate idempotency keys.

Use database transactions and constraints to protect financial records.

## 12. Money Representation

Never use floating-point numbers for money.

Use an exact representation, such as integer minor units or a decimal type.

For example:

```text
Amount: 125.50
Currency: CAD
Minor units: 12550
```

The currency must always be stored with the amount.

Do not assume that every currency uses two decimal places. Currency precision should follow the supported currency rules.

## 13. Rust Project Structure

The Payment context can follow this structure:

```text
src/
└── contexts/
    └── payment/
        ├── domain/
        │   ├── entities/
        │   │   ├── payment.rs
        │   │   ├── payment_attempt.rs
        │   │   └── refund.rs
        │   ├── value_objects/
        │   │   ├── money.rs
        │   │   ├── payment_status.rs
        │   │   └── payment_id.rs
        │   ├── errors.rs
        │   └── events.rs
        │
        ├── application/
        │   ├── commands/
        │   ├── queries/
        │   ├── services/
        │   └── ports/
        │       ├── payment_provider.rs
        │       └── payment_repository.rs
        │
        └── infrastructure/
            ├── persistence/
            ├── providers/
            └── webhooks/
```

The domain layer contains payment rules.

The application layer coordinates payment operations.

The infrastructure layer handles PostgreSQL, provider APIs, and HTTP webhook details.

The domain must not depend on Axum, SQLx, or a specific payment provider.

## 14. Testing Strategy

Payment tests must cover normal operations and failure scenarios.

### Unit tests

- Create a payment.
- Change payment status.
- Reject an invalid transition.
- Calculate the remaining refundable amount.
- Reject a refund larger than the refundable amount.
- Prevent invalid refund amounts.

### Integration tests

- Create a payment with a mock provider.
- Process a successful payment.
- Process a failed payment.
- Process a partial refund.
- Process a full refund.
- Handle provider timeouts.
- Verify webhook signatures.
- Ignore duplicate webhook events.
- Recover an unresolved payment.

### Important scenarios

| Scenario | Expected result |
|---|---|
| Customer clicks Pay twice | No duplicate charge |
| Provider confirms success | Payment becomes Succeeded |
| Provider request times out | Payment remains unresolved |
| Duplicate webhook arrives | No duplicate processing |
| Refund exceeds available amount | Request rejected |
| Unauthorized user requests refund | Access denied |

## 15. Initial Release Scope

The first version should remain simple.

Include:

- One payment provider integration.
- Hosted checkout or tokenized payment.
- Payment creation and tracking.
- Payment attempts.
- Verified webhook handling.
- Full and partial refunds.
- Idempotency.
- Basic reconciliation.
- Payment history.

Do not introduce Kafka, Redis, or a separate payment microservice.

The Payment context remains a module inside the `bio-com` modular monolith.

## 16. Future Improvements

Possible future improvements include:

- Multiple payment providers.
- Automatic provider selection.
- Advanced fraud detection.
- Payment analytics.
- Automated reconciliation.
- More advanced authorization and capture workflows.
- Multi-currency support.

These features should be introduced only when business requirements justify them.

---

## Final Design Decision

The Payment context owns all financial records and payment rules.

It integrates with external providers through application interfaces and infrastructure adapters.

Ordering owns order status, Inventory owns stock, and Payment owns financial status.

The application coordinates these contexts using explicit workflows, idempotency, verified webhooks, and recovery procedures.

This gives `bio-com` a reliable payment foundation without adding unnecessary infrastructure.
