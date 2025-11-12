# ⚡ EVerest: EV Charging Stations Microservices Platform

A cloud-native microservices system for managing and locating **Electric Vehicle (EV) charging stations**, designed with **security, scalability, and modularity** in mind.

This project provides APIs for:
- **Authentication & Authorization** (Keycloak-backed)
- **Station Discovery** (Locator-Service)
- **Station Management & Configuration** (Configurator-Service)
- **Centralized API Gateway**
- **Shared Database & Event-driven communication**



## 🧭 System Overview

### Goals
- Provide secure, token-based access for users, operators, and administrators.
- Enable geolocation-based station lookup and management.
- Support multi-tenant operator configuration and station CRUD.
- Enforce consistent security and monitoring across services.

### Core Components

| Service | Description |
|----------|--------------|
| **API Gateway** | Single entrypoint for all client APIs. Handles routing, token validation, rate-limiting, and request logging. |
| **Auth-Service** | Manages users, roles, and permissions by interfacing with **Keycloak**’s Admin API. Provides token introspection and user management. |
| **Keycloak** | Identity provider for OAuth2 / OIDC token issuance, RBAC, and identity federation. |
| **Locator-Service** | Handles geospatial queries for locating nearby charging stations and returning real-time availability data. |
| **Configurator-Service** | Provides CRUD for stations, operators, and tariffs; emits update events to synchronize Locator caches. |
| **Shared Database Service** | Centralized PostgreSQL cluster with migrations and read-replicas. Shared across internal microservices. |
| **NATS (optional)** | Lightweight event bus for inter-service communication (e.g., `station.created`, `station.updated`). |


## 🧭 System Overview
''' mermaid
    flowchart TB
        %% === Public Zone ===
        subgraph Clients["🌐 Public Zone"]
            Client["Mobile / Web Client"]
            OperatorUI["Operator UI"]
            AdminUI["Admin UI"]
        end

        