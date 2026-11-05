# Data Generation

## 1. Purpose

The data generator creates realistic sample data for the `bio-com` e-commerce platform.

It helps developers work with a populated database without manually creating customers, products, orders, payments, and other records.

The generator is used for:

- Local development.
- Functional testing.
- Integration testing.
- Demonstrations.
- Performance testing.
- Testing business workflows.

The generator is a development tool. It is not part of the production business workflow.

## 2. Technology

The data generator will be developed in Rust.

| Technology | Purpose |
|---|---|
| Rust | Generator implementation |
| Cargo | Build and dependency management |
| PostgreSQL | Target database |
| SQLx | Database access |
| Faker library | Generate realistic sample values |
| Serde | Configuration and serialization |
| Clap | Command-line interface |
| Rand | Random data generation |

The generator should reuse existing domain concepts and validation rules wherever practical.

## 3. Data Generation Principles

The generator must follow these principles:

1. Generate realistic data.
2. Preserve relationships between records.
3. Respect domain constraints.
4. Support reproducible generation.
5. Allow configurable dataset sizes.
6. Avoid creating duplicate records unintentionally.
7. Support clean database initialization.
8. Never run automatically against production databases.

Generated data must respect the same business rules as normal application data.

For example:

- An order must belong to an existing customer.
- An order must contain at least one item.
- An order item must reference a valid product or variant.
- A payment must reference an existing order.
- A refund must reference an eligible payment.
- A shipment must reference an existing order.
- Inventory quantities must respect stock constraints.

## 4. Data Categories

The generator supports the following data categories.

### 4.1 Identity and Customers

Generate:

- User accounts.
- Customer profiles.
- Customer addresses.
- Contact information.
- Account status.
- User roles and permissions where appropriate.

Use fictional names, email addresses, and telephone numbers.

Do not generate real personal information.

### 4.2 Product Catalog

Generate:

- Product categories.
- Products.
- Product descriptions.
- Product variants.
- SKUs.
- Prices.
- Product status.
- Product images using local placeholder references.

Example:

| Product | SKU | Price |
|---|---|---:|
| Wireless Mouse | ELEC-MOUSE-001 | 29.99 |
| Mechanical Keyboard | ELEC-KEY-001 | 89.99 |
| USB-C Hub | ELEC-HUB-001 | 49.99 |

### 4.3 Inventory

Generate:

- Inventory locations.
- Stock quantities.
- Reserved quantities.
- Available quantities.
- Stock movements.
- Inventory adjustments.

Inventory quantities must remain consistent with the inventory rules defined by the domain.

### 4.4 Shopping Carts

Generate:

- Active carts.
- Cart items.
- Product references.
- Quantities.
- Cart status.

Some customers should have empty carts while others should have multiple items.

### 4.5 Orders

Generate:

- Customer orders.
- Order items.
- Order totals.
- Order status.
- Billing addresses.
- Shipping addresses.
- Order creation dates.

Generate orders across different lifecycle states.

Examples:

- Pending.
- Confirmed.
- Processing.
- Shipped.
- Delivered.
- Cancelled.

Use the actual status definitions from the domain model.

### 4.6 Payments

Generate:

- Payment records.
- Payment methods.
- Payment amounts.
- Payment status.
- Transaction references.
- Payment timestamps.

Include successful, pending, and failed payments.

Payment records must remain consistent with their associated orders.

Do not generate fake successful transactions in a real payment provider.

### 4.7 Refunds

Generate:

- Refund requests.
- Refund amounts.
- Refund reasons.
- Refund status.
- Refund timestamps.

Include approved, rejected, and processed refunds where permitted by the domain rules.

Refund amounts must not exceed the refundable amount.

### 4.8 Shipping and Fulfillment

Generate:

- Shipments.
- Shipment items.
- Shipping addresses.
- Tracking references.
- Shipping methods.
- Shipment status.
- Delivery timestamps.

Generate different shipment scenarios, including pending, shipped, delivered, and failed delivery.

## 5. Relationships Between Data

Generated records must respect the relationships defined by the domain model.

Example:

```text
Customer
   |
   +---- Address
   |
   +---- Shopping Cart
   |         |
   |         +---- Cart Items
   |                    |
   |                    +---- Products
   |
   +---- Orders
              |
              +---- Order Items
              |         |
              |         +---- Products
              |
              +---- Payments
              |         |
              |         +---- Refunds
              |
              +---- Shipments
                        |
                        +---- Shipment Items
```

The generator should create records in dependency order.

It must not generate orphaned records or invalid references.

## 6. Dataset Profiles

The generator supports different dataset profiles.

| Profile | Purpose | Example |
|---|---|---|
| Minimal | Basic local development | 10 customers |
| Small | Feature development | 100 customers |
| Medium | Integration testing | 1,000 customers |
| Large | Performance testing | 10,000 customers |

The numbers are initial defaults and should be configurable.

Each profile defines the number of records generated for each category.

For example, generating 100 customers does not necessarily mean generating exactly 100 orders. The configuration should control the relationship between them.

## 7. Reproducible Generation

The generator must support a random seed.

Using the same seed and configuration should produce the same generated dataset, subject to the same generator version and initial database state.

Example:

```bash
cargo run -p data-generator -- generate \
  --profile small \
  --seed 42
```

This makes test failures easier to reproduce.

Record the generator version, seed, and configuration when producing a dataset.

## 8. Command-Line Interface

The generator should provide a simple CLI.

Example commands:

```bash
# Display available commands
cargo run -p data-generator -- --help

# Generate minimal data
cargo run -p data-generator -- generate \
  --profile minimal

# Generate a small dataset
cargo run -p data-generator -- generate \
  --profile small \
  --seed 42

# Generate a custom dataset
cargo run -p data-generator -- generate \
  --customers 500 \
  --products 200 \
  --orders 1500 \
  --seed 42

# Display available profiles
cargo run -p data-generator -- profiles

# Validate generated data
cargo run -p data-generator -- validate

# Reset development data
cargo run -p data-generator -- reset --confirm
```

The reset command must be restricted to explicitly configured development and test environments.

It must never reset a production database.

## 9. Configuration

Dataset sizes and generation behavior should be configurable.

Example `config/small.toml`:

```toml
[generator]
seed = 42

[customers]
count = 100

[products]
count = 50

[categories]
count = 8

[inventory]
locations = 2

[carts]
count = 60

[orders]
count = 250

[payments]
enabled = true

[refunds]
enabled = true

[shipping]
enabled = true
```

The generator should support configuration overrides through CLI arguments.

## 10. Project Structure

The generator belongs in the `tools` directory.

```text
tools/
└── data-generator/
    ├── Cargo.toml
    │
    ├── config/
    │   ├── minimal.toml
    │   ├── small.toml
    │   ├── medium.toml
    │   └── large.toml
    │
    ├── src/
    │   ├── main.rs
    │   ├── cli.rs
    │   ├── config.rs
    │   ├── generator/
    │   │   ├── mod.rs
    │   │   ├── customers.rs
    │   │   ├── products.rs
    │   │   ├── inventory.rs
    │   │   ├── carts.rs
    │   │   ├── orders.rs
    │   │   ├── payments.rs
    │   │   ├── refunds.rs
    │   │   └── shipping.rs
    │   │
    │   ├── persistence/
    │   │   ├── mod.rs
    │   │   └── database.rs
    │   │
    │   └── validation/
    │       ├── mod.rs
    │       └── integrity.rs
    │
    └── tests/
        ├── generation_tests.rs
        └── integrity_tests.rs
```

## 11. Database Access

The generator writes data into PostgreSQL.

It must follow the database schema and respect foreign keys, unique constraints, and check constraints.

There are two supported approaches:

**Application-level generation**

Use existing application services or use cases to create records.

This is preferred when testing business workflows and domain behavior.

**Direct database generation**

Use SQLx to insert large datasets efficiently.

This is useful for development fixtures and performance datasets.

Direct database generation must not bypass required invariants or create invalid business states.

For bulk generation, transaction boundaries should be configurable.

## 12. Validation

After generation, the tool should validate the resulting dataset.

Examples:

- Every order references an existing customer.
- Every order item references an existing product.
- Every payment references an existing order.
- Every refund references an existing payment.
- Every shipment references an existing order.
- No duplicate SKUs exist.
- Monetary totals are consistent.
- Inventory quantities are valid.
- Foreign key relationships are intact.

The validation command should return a non-zero exit code when integrity checks fail.

## 13. Safety

The data generator must include safeguards.

- Never use production database credentials.
- Require explicit environment configuration.
- Restrict reset operations to development and test databases.
- Avoid real customer personal information.
- Avoid calling real payment gateways.
- Avoid sending real emails or shipping requests.
- Use transactions where appropriate.
- Log generation errors without exposing credentials.

The generator should default to the local development database.

## 14. Testing Strategy

### Unit Tests

Test individual generators.

Examples:

- Customer generation.
- Product generation.
- Price generation.
- Order status generation.
- Refund amount generation.

### Integration Tests

Test database persistence and relationships.

Examples:

- Generate customers and products.
- Generate orders with order items.
- Generate payments for existing orders.
- Generate valid refunds.
- Generate shipments.

### Reproducibility Tests

Verify that the same seed and configuration produce equivalent datasets.

### Integrity Tests

Verify that generated records satisfy database and domain constraints.

## 15. Architectural Decisions

| Area | Decision |
|---|---|
| Language | Rust |
| Location | `tools/data-generator/` |
| Database | PostgreSQL |
| Database access | SQLx |
| CLI | Clap |
| Configuration | TOML |
| Random generation | Seeded RNG |
| Data profiles | Minimal, Small, Medium, Large |
| Validation | Domain and database integrity |
| Environment | Development and testing |
| Production | Not permitted |

## 16. Design Principles

1. Keep the generator separate from production business workflows.
2. Reuse domain concepts and validation rules.
3. Generate consistent relationships.
4. Support deterministic datasets.
5. Make dataset sizes configurable.
6. Protect production data.
7. Support both realistic scenarios and large datasets.
8. Keep the implementation simple.
9. Make generated data easy to reset and reproduce.
10. Validate data integrity after generation.

## 17. Future Improvements

Potential improvements include:

- Scenario-based generation.
- Importing product catalogs from CSV.
- Generating datasets from JSON fixtures.
- Generating customer purchase histories.
- Simulating order lifecycle transitions.
- Generating concurrent workloads.
- Measuring database performance.
- Generating synthetic datasets for analytics.

These capabilities can be introduced as the project grows.
