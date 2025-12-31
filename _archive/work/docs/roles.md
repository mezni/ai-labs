# 🔐 Roles and Permissions

This document defines the roles, permissions, and access scopes for all services in the DDD-based platform.

---

## 👥 User Roles Overview

| Role Key | Role Name | Description | Auth Required | Typical Access |
|-----------|------------|--------------|----------------|----------------|
| **SYS_ADMIN** | **System Administrator** | Full platform control — manages users, roles, configurations, and operational rules. | ✅ Yes | All services |
| **OPS_SUPERVISOR** | **Operator Supervisor** | Oversees operators, validates data operations, and monitors batch execution. | ✅ Yes | Service A, partial Service B |
| **OPS_AGENT** | **Operations Agent** | Performs daily operational tasks such as uploading, validating, and monitoring data. | ✅ Yes | Service A |
| **REGISTERED_USER** | **Registered Analyst / Customer** | Authenticated user who can view personal dashboards and reports. | ✅ Yes | Service B |
| **PUBLIC_USER** | **Guest / Unregistered User** | Anonymous or external user with access to public endpoints only. | ❌ No | Limited public APIs |

---

## 🧩 Role Capability Matrix

| Capability | System Admin | Operator Supervisor | Operator Agent | Registered User | Public User |
|-------------|---------------|----------------------|----------------|------------------|--------------|
| **Manage Users & Roles** | ✅ Full | ❌ | ❌ | ❌ | ❌ |
| **Approve or Reject Data Uploads** | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Upload Data Files** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Validate & Normalize Data** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Monitor Batch Executions** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Manage Rules & Thresholds** | ✅ | ✅ | ❌ | ❌ | ❌ |
| **View Analytics Dashboard** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Access Notifications** | ✅ | ✅ | ✅ | ✅ (own only) | ❌ |
| **Modify System Configurations** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **View Public Info (Status, Docs)** | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## ⚙️ Role Permissions by Service

### 🔸 Auth-Service
| Endpoint | Method | Role Access |
|-----------|---------|-------------|
| `/api/v1/auth/register` | POST | Public User |
| `/api/v1/auth/login` | POST | Public User |
| `/api/v1/auth/users` | GET, POST, DELETE | System Admin |
| `/api/v1/auth/roles` | GET, POST | System Admin |
| `/api/v1/auth/profile` | GET | All Authenticated Roles |
| `/api/v1/auth/tokens/introspect` | POST | All Authenticated Roles |

---

### 🔸 Service A (Data Management & Operations)
| Endpoint | Method | Role Access |
|-----------|---------|-------------|
| `/api/v1/service-a/upload` | POST | Operator Agent, Operator Supervisor, System Admin |
| `/api/v1/service-a/validate` | POST | Operator Supervisor, System Admin |
| `/api/v1/service-a/batch/status` | GET | Operator Agent, Operator Supervisor, System Admin |
| `/api/v1/service-a/rules` | POST, PUT | Operator Supervisor, System Admin |
| `/api/v1/service-a/public/info` | GET | Public User |

---

### 🔸 Service B (Analytics & Reporting)
| Endpoint | Method | Role Access |
|-----------|---------|-------------|
| `/api/v1/service-b/analytics` | GET | All Authenticated Roles |
| `/api/v1/service-b/notifications` | GET | Operator Agent, Operator Supervisor, System Admin |
| `/api/v1/service-b/overview` | GET | Registered User, Operator Supervisor, System Admin |
| `/api/v1/service-b/public/dashboard` | GET | Public User |

---

## 🧠 Role Hierarchy

```mermaid
graph TD
    A[System Administrator] --> B[Operator Supervisor]
    B --> C[Operator Agent]
    C --> D[Registered User]
    D --> E[Public User]
