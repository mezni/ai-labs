use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    info!("Database connection pool created");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(20) PRIMARY KEY,
            keycloak_id VARCHAR(255) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            source VARCHAR(20) NOT NULL,
            roles TEXT[] NOT NULL DEFAULT '{}',
            network_id VARCHAR(20),
            station_id VARCHAR(20),
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            deleted_at TIMESTAMPTZ
        );

        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
        CREATE INDEX IF NOT EXISTS idx_users_keycloak_id ON users(keycloak_id);
        CREATE INDEX IF NOT EXISTS idx_users_deleted_at ON users(deleted_at);

        CREATE TABLE IF NOT EXISTS audit_logs (
            id VARCHAR(20) PRIMARY KEY,
            user_id VARCHAR(20),
            action VARCHAR(100) NOT NULL,
            resource_type VARCHAR(50) NOT NULL,
            resource_id VARCHAR(20),
            details JSONB,
            ip_address VARCHAR(45),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
        CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
        "#,
    )
    .execute(pool)
    .await?;

    info!("Database migrations completed");
    Ok(())
}