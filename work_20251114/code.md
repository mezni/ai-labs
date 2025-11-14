## architecture

flowchart TB
    %% === Public Zone ===
    subgraph Clients["🌐 Public Zone (Web/Mobile)"]
        Client["User"]
        OperatorUI["Operator"]
        AdminUI["Admin"]
    end

    %% === Gateway Zone ===
    subgraph Gateway["🧭 API Gateway Zone"]
        APIGW["API Gateway<br/>(Tyk)<br/><small>Routing • AuthN • Rate-limit</small>"]
    end

    %% === Security Zone ===
    subgraph Security["🔐 Identity & Auth Zone"]
        Auth["Auth-Service<br/><small>(Integrates with Keycloak)</small>"]
        AuthCache["Auth-Cache<br/><small>(JWT/Introspection cache)</small>"]
        KC["Keycloak<br/><small>OIDC / OAuth2 Provider</small>"]
        KD["Keycloak DB"]
    end    

    %% === Internal Services Zone ===
    subgraph Internal["⚙️ Internal Microservices Zone"]
        MS1["Locator-Service<br/><small>Find nearby stations</small>"]
        MS2["Configurator-Service<br/><small>Manage stations, operators, tariffs</small>"]
        SharedDB["PostgreSQL<br/><small>Shared Data Storage</small>"]
    end

    %% === Client to Gateway Flows ===
    Client -->|"HTTPS (OIDC tokens)"| APIGW
    OperatorUI -->|"HTTPS (JWT)"| APIGW
    AdminUI -->|"HTTPS (JWT)"| APIGW

    %% === Gateway to Security Zone ===
    APIGW -->|Auth request / Login| Auth
    Auth -->|Request token| KC 
    Auth -->|Token check| AuthCache       
    KC -->|Read/Write user data| KD

    %% === Gateway to Internal Services ===
    APIGW -->|Validated JWT| MS1
    APIGW -->|Validated JWT| MS2

    %% === Internal Services to DB ===
    MS1 -->|Read / Query| SharedDB
    MS2 -->|CRUD / Config| SharedDB


    %% === Class Styling ===
    classDef external fill:#f9f9f9,stroke:#666,stroke-width:1px;
    classDef gateway fill:#fff7e6,stroke:#ffb703,stroke-width:2px;
    classDef security fill:#e3f2fd,stroke:#2196f3,stroke-width:2px;
    classDef internal fill:#f1f8e9,stroke:#4caf50,stroke-width:2px;
    classDef data fill:#fff3e0,stroke:#fb8c00,stroke-width:2px;

    class Clients external;
    class Gateway gateway;
    class Security security;
    class Internal internal;
    class SharedDB data;



## ERD

erDiagram
    NETWORKS ||--o{ COMPANIES : has
    NETWORKS ||--o{ STATIONS : operates
    STATIONS ||--o{ CONNECTORS : contains
    STATIONS ||--o{ STATION_VERIFICATION_HISTORY : has_history
    CONNECTORS }o--|| CONNECTOR_TYPES : uses
    STATIONS ||--o{ MV_NEARBY_STATIONS_SUMMARY : summarized_in
    STATIONS ||--o{ MV_NEARBY_STATIONS_DETAILES : detailed_in
    PLANET_OSM_POINT ||--o{ MV_NEARBY_STATIONS_SUMMARY : reference
    PLANET_OSM_POINT ||--o{ MV_NEARBY_STATIONS_DETAILES : reference
