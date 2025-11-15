# 🧩 User Stories

This document defines user stories, acceptance criteria, and priorities for all microservices in the system.

---

## 📚 Overview

The platform consists of:
- **Auth-Service** → Authentication, roles, and access control (Keycloak integration)
- **Service A** → Core data ingestion and processing
- **Service B** → Analytics and reporting
- **Shared Database** → Centralized schema for DDD entities

Each service contributes to the domain model and user workflows described below.

---

## 🗺️ Epics

| Epic ID | Epic Name | Description | Owner | Status |
|----------|------------|-------------|--------|---------|
| EP01 | Authentication & Authorization | Enable secure user login and role-based access | Auth Team | ✅ Done |
| EP02 | Data Management | Manage and process business data | Service A | 🟡 In Progress |
| EP03 | Analytics Dashboard | Provide metrics and KPIs visualization | Service B | ⏳ Planned |

---

## 👤 User Roles

| Role | Description |
|------|--------------|
| **Admin** | Manages users, roles, and global configuration |
| **Operator** | Runs daily operational workflows |
| **Viewer** | Has read-only access to analytics and dashboards |

---

## 🧠 User Stories

### 🔐 **Epic: Authentication & Authorization (EP01)**

#### Story: User Registration
**As an** admin  
**I want** to register new users with email and password  
**So that** they can access the system securely  

**Acceptance Criteria:**
- [ ] API endpoint `/api/v1/auth/register` available
- [ ] Passwords hashed using Argon2
- [ ] Duplicate emails rejected
- [ ] Response returns JWT token and user profile  

**Priority:** ⭐ High  
**Status:** ✅ Completed  

---

#### Story: Login & Token Generation
**As a** registered user  
**I want** to log in and receive a JWT token  
**So that** I can authenticate my subsequent requests  

**Acceptance Criteria:**
- [ ] Endpoint `/api/v1/auth/login`
- [ ] Valid credentials return access & refresh tokens
- [ ] Invalid credentials return `401 Unauthorized`
- [ ] Tokens have a 15-minute expiry  

**Priority:** ⭐⭐ High  
**Status:** ✅ Completed  

---

### 📦 **Epic: Data Management (EP02)**

#### Story: Upload Data File
**As an** operator  
**I want** to upload a CSV file containing daily operational data  
**So that** it can be parsed and stored in the shared database  

**Acceptance Criteria:**
- [ ] Endpoint `/api/v1/service-a/upload`
- [ ] Supports `.csv` and `.json`
- [ ] Data validated before insertion
- [ ] File metadata logged in `batch_execs` table  

**Priority:** ⭐⭐ High  
**Status:** 🟡 In Progress  

---

#### Story: Validate & Normalize Data
**As a** system  
**I want** to validate and normalize country and operator fields  
**So that** data quality and integrity are maintained  

**Acceptance Criteria:**
- [ ] Normalize country codes to ISO-3166
- [ ] Reject malformed records
- [ ] Maintain audit logs  

**Priority:** ⭐ Medium  
**Status:** 🟡 In Progress  

---

### 📊 **Epic: Analytics Dashboard (EP03)**

#### Story: View Roamers by Country
**As a** viewer  
**I want** to visualize the number of roamers by country  
**So that** I can analyze traffic distribution  

**Acceptance Criteria:**
- [ ] API endpoint `/api/v1/service-b/roamers`
- [ ] Data aggregated by `dim_countries`
- [ ] Chart rendered on dashboard  
- [ ] Supports filters by date range  

**Priority:** ⭐⭐ High  
**Status:** ⏳ Planned  

---

#### Story: Receive Anomaly Notifications
**As a** network analyst  
**I want** to receive notifications when anomaly thresholds are exceeded  
**So that** I can take immediate corrective actions  

**Acceptance Criteria:**
- [ ] Threshold rules defined in `rules` table
- [ ] Notifications stored in `notifications` table
- [ ] API endpoint `/api/v1/notifications`
- [ ] Email integration (future phase)  

**Priority:** ⭐⭐ High  
**Status:** ⏳ Planned  

---

## 🧾 Non-Functional Requirements

| Area | Requirement |
|------|--------------|
| **Performance** | Each service must handle ≥ 500 requests/second |
| **Security** | Use JWT, HTTPS, and minimal privilege principles |
| **Resilience** | Automatic retry and backoff for DB writes |
| **Observability** | Include structured logging and metrics (Prometheus) |

---

## 🧭 Traceability Matrix

| User Story ID | Service | Endpoint | DDD Layer | Status |
|----------------|----------|-----------|-------------|--------|
| US01 | auth-service | `/register` | `interfaces/application` | ✅ |
| US02 | auth-service | `/login` | `interfaces/application` | ✅ |
| US03 | service-a | `/upload` | `interfaces/infrastructure` | 🟡 |
| US04 | service-b | `/roamers` | `interfaces/application` | ⏳ |
| US05 | service-b | `/notifications` | `interfaces/application` | ⏳ |

---

## 🖼️ Diagrams

> Located in `docs/images/` directory.

- `images/user_roles.png` — System roles & permissions
- `images/user_flow.png` — Registration & login flow
- `images/data_flow.png` — File upload to analytics pipeline

Example (Mermaid):
```mermaid
sequenceDiagram
  participant U as User
  participant A as Auth-Service
  participant S as Service A
  participant D as Database

  U->>A: POST /login
  A-->>U: JWT token
  U->>S: POST /upload (Authorization: Bearer token)
  S->>D: Insert normalized data
  S-->>U: Upload success
