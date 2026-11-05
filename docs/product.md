# bio-com — Product Overview

## 1. What is bio-com?

**bio-com** is an e-commerce platform for selling natural and organic products.

The platform focuses on products such as:

- Olive oil
- Honey
- Herbal essences
- Natural wellness products
- Agricultural and related products

The goal is to build a reliable online store where customers can discover products, place orders, and track their purchases.

At the same time, administrators and operational staff need tools to manage products, stock, orders, payments, and deliveries.

We want a platform that is simple to use, easy to maintain, and ready to grow as the business expands.

## 2. Product Vision

Our vision is to build a complete e-commerce platform that supports the daily operations of a natural products business.

The platform should provide:

- A simple shopping experience for customers.
- Clear and accurate product information.
- Reliable inventory and order management.
- Secure customer accounts and payments.
- Easy tools for administrators and staff.
- Product traceability, including origin and batch information.

We want to start with a solid foundation rather than introduce unnecessary complexity.

## 3. Who Will Use the Platform?

The platform has three main types of users.

### 3.1 Customers

Customers use the platform to:

- Browse available products.
- Search and filter products.
- View product details, prices, and availability.
- Add products to their shopping cart.
- Place and pay for orders.
- Manage their personal information.
- View order history.
- Track order status.

### 3.2 Administrators

Administrators manage the business through a back-office interface.

They can:

- Manage products and categories.
- Set prices and promotions.
- Manage customer accounts.
- Manage staff access and permissions.
- Monitor orders and payments.
- View inventory information.
- Access business reports.

### 3.3 Operational Staff

Operational staff handle day-to-day activities.

Depending on their permissions, they can:

- Update inventory.
- Manage product batches.
- Prepare customer orders.
- Update fulfillment status.
- Handle returns and order issues.
- Check product availability.

## 4. Main Features

The platform is organized around the following business capabilities.

### 4.1 Product Catalog

The catalog contains all products available in the store.

Each product should have information such as:

- Name and description.
- Product category.
- Images.
- Price.
- Available variants.
- Ingredients and characteristics.
- Country or region of origin.
- Certifications, when applicable.
- Product status (active or inactive).

For example, olive oil may have different sizes, packaging options, and origins.

### 4.2 Pricing and Promotions

The platform must support product pricing and promotional offers.

Main capabilities:

- Define product prices.
- Support different product variants.
- Create discounts and promotions.
- Apply promotional rules.
- Calculate order totals.
- Keep a clear record of applied discounts.

### 4.3 Shopping Cart

Customers need a shopping cart to prepare their orders.

The cart should allow them to:

- Add products.
- Change quantities.
- Remove products.
- Review prices.
- See the estimated order total.

Product availability and prices must be checked again during checkout.

### 4.4 Customer Accounts

Customers can create accounts and manage their information.

The platform should support:

- Registration and login.
- Profile management.
- Delivery addresses.
- Order history.
- Account status management.

### 4.5 Identity and Access Management

The platform needs its own identity and access management module.

It will handle:

- User authentication.
- Password management.
- User roles.
- Permissions.
- Access control.
- Account activation and deactivation.

Access must depend on the user's role and permissions.

For example, a customer must not have access to administrative operations.

### 4.6 Order Management

Order management handles the complete order lifecycle.

The platform should support:

- Creating orders.
- Validating order details.
- Confirming product availability.
- Calculating totals.
- Tracking order status.
- Managing cancellations.
- Handling returns and refunds.

An order must have a clear status throughout its lifecycle.

### 4.7 Inventory Management

Inventory management keeps track of available products.

Main capabilities:

- Track available quantities.
- Receive new stock.
- Update stock levels.
- Reserve products for orders.
- Release reservations when necessary.
- Record inventory movements.
- Prevent sales when stock is insufficient.

Inventory must remain consistent when multiple customers purchase the same product.

### 4.8 Batch Management and Traceability

Some products require additional information about their production and origin.

The platform should support:

- Batch identification.
- Production dates.
- Expiration dates, when applicable.
- Product origin.
- Supplier information.
- Batch-level stock tracking.
- Traceability from stock to customer order.

This is particularly important for products such as olive oil, honey, and herbal essences.

### 4.9 Payment Management

The platform must support secure customer payments.

Main capabilities:

- Initiate payments.
- Track payment status.
- Confirm successful payments.
- Handle failed payments.
- Support refunds.
- Keep payment records.

Payment processing should be handled through an external payment provider.

The platform must never store raw card information.

### 4.10 Order Fulfillment

Fulfillment covers the activities required to prepare and deliver an order.

The platform should support:

- Preparing orders.
- Picking and packing products.
- Recording shipment information.
- Updating delivery status.
- Tracking fulfillment progress.
- Handling delivery problems.

### 4.11 Compliance and Product Information

The platform should provide accurate product information and support business traceability requirements.

This includes:

- Product origin.
- Ingredients.
- Product characteristics.
- Certifications and supporting information.
- Batch records.
- Expiration information, when applicable.

The platform should make this information available to customers and authorized staff.

## 5. Main Business Workflow

The typical customer journey looks like this:

1. The customer visits the online store.
2. The customer browses the catalog.
3. The customer selects products.
4. The customer adds products to the cart.
5. The customer reviews the cart.
6. The customer provides delivery information.
7. The platform validates prices and inventory.
8. The customer completes payment.
9. The platform confirms the order.
10. Inventory is updated.
11. Staff prepare the order.
12. The order is shipped.
13. The customer receives the order.

Each step must have clear business rules.

For example, the platform must not confirm an order when the required stock is unavailable or the payment has failed.

## 6. Initial Product Scope

We will build the platform incrementally.

The first release should focus on the essential features required to operate an online store.

### Initial release

- Customer registration and login.
- Role-based access control.
- Product catalog and categories.
- Product variants and pricing.
- Shopping cart.
- Checkout.
- Inventory management.
- Order management.
- Payment integration.
- Basic fulfillment management.
- Administrative interface.

### Features to expand later

- Advanced promotions.
- Detailed product traceability.
- Advanced reporting and analytics.
- Supplier management.
- Loyalty programs.
- More advanced inventory operations.
- Additional payment and delivery integrations.

The goal is to deliver a working business platform first, then expand it based on real business needs.

## 7. What We Are Not Building Initially

To keep the first version manageable, we will avoid unnecessary complexity.

The initial product will not include:

- A microservices architecture.
- A distributed messaging platform.
- Complex event-streaming infrastructure.
- A dedicated distributed caching system.
- Kubernetes-based deployment.
- A large external identity management platform.
- Advanced analytics infrastructure.

These capabilities can be considered later when there is a clear business or technical reason.

## 8. Product Principles

The following principles will guide product development.

**Simplicity first**

Build what the business needs. Avoid unnecessary features and complexity.

**Reliable business operations**

Orders, payments, and inventory must behave consistently.

**Security by design**

Customer information and administrative operations must be protected.

**Clear product information**

Customers should understand what they are buying, including product origin and characteristics.

**Traceability**

Important product and inventory information should be recorded and accessible when needed.

**Maintainability**

The platform should be easy to understand, test, modify, and extend.

**Room to grow**

Start with a manageable architecture without preventing future improvements.

## 9. Technical Direction

The product will be built using the following technical decisions.

| Area | Decision |
|---|---|
| Programming language | Rust |
| Architecture | Clean Architecture |
| Design approach | Domain-Driven Design (DDD) |
| Application structure | Modular Monolith |
| Database | PostgreSQL |
| Database access | SQLx |
| Backend framework | Axum |
| Identity and access | Internal Rust application module |
| Authorization | Role-Based Access Control (RBAC) |
| Infrastructure | Lightweight |
| Deployment | Simple deployment initially |

The application will be organized into business modules with clear responsibilities.

Each module will own its business rules and data.

We will use PostgreSQL as the main database and avoid introducing additional infrastructure unless it becomes necessary.

## 10. Definition of Success

We consider the initial product successful when:

- Customers can register and browse products.
- Customers can place orders successfully.
- Payments can be processed and tracked.
- Inventory remains consistent.
- Staff can manage orders and fulfillment.
- Administrators can manage products and users.
- Access is controlled by roles and permissions.
- Product and batch information can be tracked.
- The application can be tested, deployed, and maintained reliably.

The objective is not simply to build an online store.

It is to build a reliable business platform that can support real operations today and grow with the business tomorrow.

---

**Project:** bio-com  
**Language:** Rust  
**Architecture:** DDD + Clean Architecture  
**Application Style:** Modular Monolith  
**Database:** PostgreSQL
