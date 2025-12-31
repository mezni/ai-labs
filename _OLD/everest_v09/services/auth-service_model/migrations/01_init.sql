CREATE TABLE IF NOT EXISTS users (
    user_id         VARCHAR(32) PRIMARY KEY,
    keycloak_id     VARCHAR(255) UNIQUE NOT NULL,

    email           VARCHAR(255) UNIQUE NOT NULL,
    username        VARCHAR(100) UNIQUE NOT NULL,

    first_name      VARCHAR(100),
    last_name       VARCHAR(100),
    phone           VARCHAR(20),
    photo           TEXT,

    is_verified     BOOLEAN DEFAULT TRUE,
    role            VARCHAR(50) NOT NULL DEFAULT 'user',

    network_id      VARCHAR(32) NOT NULL DEFAULT '',
    station_id     VARCHAR(32) NOT NULL DEFAULT '',
    source          VARCHAR(20) NOT NULL DEFAULT 'web',

    is_active       BOOLEAN DEFAULT TRUE,
    deleted_at      TIMESTAMPTZ,
    last_login_at   TIMESTAMPTZ,

    created_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    created_by      VARCHAR(32),
    updated_by      VARCHAR(32),

    CONSTRAINT valid_role CHECK (
        role IN ('user', 'admin', 'partner', 'operator')
    ),
    CONSTRAINT valid_source CHECK (
        source IN ('web', 'mobile', 'internal')
    ),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE)
        OR
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);


CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id     VARCHAR(32) PRIMARY KEY,

    email               VARCHAR(255) NOT NULL,
    username            VARCHAR(100) NOT NULL,

    first_name          VARCHAR(100),
    last_name           VARCHAR(100),
    phone               VARCHAR(20),

    verification_token  VARCHAR(255) NOT NULL,
    status              VARCHAR(20) NOT NULL DEFAULT 'pending',

    keycloak_id         VARCHAR(255) UNIQUE NOT NULL,
    user_id             VARCHAR(32),

    resend_count        INTEGER DEFAULT 0,
    expires_at          TIMESTAMPTZ NOT NULL,
    verified_at         TIMESTAMPTZ,

    created_at          TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    ip_address          VARCHAR(50),
    user_agent          TEXT,
    source              VARCHAR(20) NOT NULL DEFAULT 'web',

    CONSTRAINT valid_registration_status CHECK (
        status IN ('pending', 'verified', 'expired', 'cancelled')
    ),
    CONSTRAINT fk_registration_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE SET NULL
);
