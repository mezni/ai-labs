ev-platform/
│
├── services/
│   ├── auth-service/
│   │   ├── src/
│   │   │   ├── api/                          # REST + OpenAPI
│   │   │   │   ├── handlers/
│   │   │   │   │   ├── user_handler.rs
│   │   │   │   │   └── auth_handler.rs
│   │   │   │   ├── routes.rs
│   │   │   │   ├── dto.rs
│   │   │   │   └── openapi.rs
│   │   │   │
│   │   │   ├── application/                 # CQRS: Commands & Queries
│   │   │   │   ├── commands/
│   │   │   │   │   ├── register_user.rs
│   │   │   │   │   └── assign_role.rs
│   │   │   │   ├── queries/
│   │   │   │   │   ├── get_user.rs
│   │   │   │   │   └── list_roles.rs
│   │   │   │   └── services/
│   │   │   │       └── auth_app_service.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── entities/
│   │   │   │   │   ├── user.rs
│   │   │   │   │   └── role.rs
│   │   │   │   ├── value_objects/
│   │   │   │   │   └── email.rs
│   │   │   │   ├── repositories/
│   │   │   │   │   └── user_repository.rs
│   │   │   │   └── services/
│   │   │   │       └── domain_auth_service.rs
│   │   │   │
│   │   │   ├── infrastructure/
│   │   │   │   ├── db/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── postgres_repository.rs
│   │   │   │   ├── security/
│   │   │   │   │   ├── jwt.rs
│   │   │   │   │   └── password.rs
│   │   │   │   ├── config.rs
│   │   │   │   ├── logger.rs
│   │   │   │   ├── errors.rs
│   │   │   │   └── telemetry.rs
│   │   │   │
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── configurator-service/
│   │   ├── src/
│   │   │   ├── api/
│   │   │   ├── application/
│   │   │   │   ├── commands/
│   │   │   │   │   ├── create_station.rs
│   │   │   │   │   ├── update_station.rs
│   │   │   │   │   └── delete_station.rs
│   │   │   │   ├── queries/
│   │   │   │   │   ├── list_stations.rs
│   │   │   │   │   └── get_station.rs
│   │   │   │   └── services/
│   │   │   │       └── station_app_service.rs
│   │   │   ├── domain/
│   │   │   │   ├── entities/...
│   │   │   │   ├── repositories/...
│   │   │   │   └── services/
│   │   │   │       └── audit_service.rs
│   │   │   ├── infrastructure/...
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   └── locator-service/
│       ├── src/
│       │   ├── api/
│       │   ├── application/
│       │   │   ├── queries/
│       │   │   │   ├── find_nearest_station.rs
│       │   │   │   └── get_station_by_id.rs
│       │   │   └── services/
│       │   │       └── locator_app_service.rs
│       │   ├── domain/
│       │   │   ├── entities/station.rs
│       │   │   └── repositories/station_repository.rs
│       │   ├── infrastructure/...
│       │   └── main.rs
│       ├── Cargo.toml
│       └── README.md
│
├── shared/
│   ├── src/
│   │   ├── dto/
│   │   ├── models/
│   │   ├── errors.rs
│   │   ├── security/
│   │   │   └── jwt_claims.rs
│   │   ├── utils/
│   │   └── mod.rs
│   ├── Cargo.toml
│   └── README.md
│
├── deployment/
│   ├── docker/
│   │   ├── docker-compose.yml
│   │   ├── Dockerfile.auth
│   │   ├── Dockerfile.configurator
│   │   ├── Dockerfile.locator
│   │   └── Dockerfile.base
│   │
│   ├── kubernetes/
│   │   ├── base/
│   │   │   ├── postgres-deployment.yaml
│   │   │   ├── postgres-service.yaml
│   │   │   └── secrets.yaml
│   │   ├── auth-service.yaml
│   │   ├── configurator-service.yaml
│   │   ├── locator-service.yaml
│   │   └── ingress.yaml
│   │
│   └── README.md
│
├── docs/
│   ├── architecture.md
│   ├── security.md
│   ├── development-guide.md
│   ├── user-stories.md
│   ├── roles-and-permissions.md
│   └── assets/
│       ├── diagrams/
│       │   ├── ddd_layers.png
│       │   ├── cqrs_architecture.png
│       │   └── swagger_sequence.png
│       └── icons/
│
├── scripts/
│   ├── run-local.sh
│   ├── test.sh
│   └── migrate.sh
│
├── Makefile
└── README.md

src/
└── shared/
    ├── cqrs/
    │   ├── command.rs
    │   ├── handler.rs
    │   ├── mediator.rs
    │   └── query.rs
    ├── dto/
    │   └── response_dto.rs
    ├── errors/
    │   └── app_error.rs
    ├── result/
    │   └── mod.rs
    ├── traits/
    │   └── repository.rs
    ├── utils/
    │   ├── date_utils.rs
    │   └── validators.rs
    └── mod.rs


use crate::shared::{
    dto::response_dto::ApiResponse,
    errors::app_error::AppError,
    result::AppResult,
    utils::date_utils::now_utc,
};

async fn example_handler() -> AppResult<ApiResponse<String>> {
    let timestamp = now_utc();
    Ok(ApiResponse::success(format!("Current time: {}", timestamp)))
}




src/
├── main.rs
├── lib.rs
│
├── domain/
│   ├── entities/
│   │   └── network.rs
│   ├── repositories/
│   │   └── network_repository.rs
│   └── mod.rs
│
├── application/
│   ├── commands/
│   │   └── create_network.rs
│   ├── queries/
│   │   └── get_network_by_id.rs
│   ├── services/
│   │   └── network_service.rs
│   └── mod.rs
│
├── infrastructure/
│   ├── persistence/
│   │   └── postgres_network_repository.rs
│   ├── config.rs
│   └── mod.rs
│
├── presentation/
│   ├── handlers/
│   │   └── network_handler.rs
│   ├── routes.rs
│   ├── swagger.rs
│   └── mod.rs
│
└── shared/
    ├── errors.rs
    ├── result.rs
    ├── bus.rs
    ├── dto.rs
    ├── traits.rs
    ├── uuid.rs
    ├── date_time.rs
    └── mod.rs


1) Workspace layout (recommended)