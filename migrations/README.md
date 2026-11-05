Database Migrations
===================

This directory contains the PostgreSQL schema migrations for bio-com.

Naming convention
Use sequential migration numbers and descriptive names.
0001_create_customers.sql
0002_create_products.sql
0003_create_orders.sql
0004_create_order_items.sql

Rules
1. Every schema change must have a migration.
2. Never modify a migration already applied to a shared environment.
3. Use a new migration to modify an existing table.
4. Add appropriate primary keys, foreign keys, unique constraints and indexes.
5. Use NUMERIC for monetary values.
6. Use TIMESTAMPTZ for timestamps.
7. Avoid destructive schema changes without a migration and data-migration plan.
8. Review migrations before applying them to production.

Rollback strategy
SQLx migrations are tracked and executed in order.

For production, we prefer forward-only corrective migrations.

For example, if migration 0003 introduces an incorrect column, create a new migration that corrects the schema rather than rewriting migration history.

Before destructive changes, take appropriate backups and define a recovery procedure.