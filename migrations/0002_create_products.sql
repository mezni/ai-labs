CREATE TABLE products (
    id UUID PRIMARY KEY,

    sku VARCHAR(100) NOT NULL,

    name VARCHAR(255) NOT NULL,

    description TEXT,

    price NUMERIC(12, 2) NOT NULL,

    currency CHAR(3) NOT NULL,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_products_sku UNIQUE (sku),

    CONSTRAINT chk_products_price
        CHECK (price >= 0),

    CONSTRAINT chk_products_currency
        CHECK (currency ~ '^[A-Z]{3}$')
);

CREATE INDEX idx_products_active
    ON products (is_active);

CREATE INDEX idx_products_created_at
    ON products (created_at);