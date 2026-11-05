# bio-com

A reliable e-commerce platform for selling natural and organic products, built with Rust, Domain-Driven Design, and a modular monolith architecture.

## Overview

bio-com provides a complete online store experience for natural products businesses, including:

- **Customer features**: product browsing, shopping cart, checkout, order tracking, account management
- **Administrator tools**: product/catalog management, inventory control, order/fullfilment management, customer and refund management
- **Payment processing**: secure payment provider integration with webhook handling and refund support
- **Inventory management**: stock tracking, reservations, concurrency protection
- **Fulfillment**: shipment creation, tracking, delivery status management
- **Traceability**: product batch and origin tracking

## Architecture

- **Language**: Rust
- **Architecture**: DDD + Clean Architecture
- **Application Style**: Modular Monolith
- **Database**: PostgreSQL
- **API**: REST with JSON, Axum framework
- **Authentication**: Database-backed sessions, Argon2id password hashing, RBAC
- **Deployment**: Docker, Docker Compose for local development

## Core Concepts

The platform is organized into bounded contexts with clear responsibilities:

- Identity & Access
- Customer
- Catalog
- Pricing
- Cart
- Inventory
- Ordering
- Payment
- Fulfillment
- Traceability

## Getting Started

See the [docs](/docs) directory for detailed documentation on architecture, domain models, APIs, and implementation guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE.md](/LICENSE.md) file for details.