CREATE TABLE customers (
    id UUID PRIMARY KEY,

    email VARCHAR(255) NOT NULL,

    first_name VARCHAR(100) NOT NULL,

    last_name VARCHAR(100) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_customers_email UNIQUE (email)
);

CREATE INDEX idx_customers_created_at
    ON customers (created_at);