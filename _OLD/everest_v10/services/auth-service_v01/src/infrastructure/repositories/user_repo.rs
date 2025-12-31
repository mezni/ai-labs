use crate::core::errors::AppResult;
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &User) -> AppResult<User> {
        // Note: email_normalized and username_normalized are omitted
        // because they are GENERATED ALWAYS columns.
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                user_id, keycloak_id, email, username, first_name, last_name,
                phone, photo, is_verified, role, network_id, station_id,
                source, is_active, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(&user.source)
        .bind(user.is_active)
        .bind(&user.created_by)
        .bind(&user.updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
        // Using 'is_active = TRUE' to match your check_deleted constraint
        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE user_id = $1 AND is_active = TRUE",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        // Querying against the base email; Postgres will handle the comparison
        let result =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_active = TRUE")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?;

        Ok(result)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE keycloak_id = $1 AND is_active = TRUE",
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $2, 
                username = $3, 
                first_name = $4, 
                last_name = $5,
                phone = $6, 
                photo = $7, 
                is_verified = $8, 
                role = $9,
                network_id = $10, 
                station_id = $11, 
                is_active = $12,
                updated_by = $13
            WHERE user_id = $1
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(&user.updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update_last_login(&self, user_id: &str) -> AppResult<()> {
        sqlx::query("UPDATE users SET last_login_at = CURRENT_TIMESTAMP WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list_active(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users 
            WHERE is_active = TRUE 
            ORDER BY created_at DESC 
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    async fn count_active(&self, network_id: Option<&str>) -> AppResult<i64> {
        let query = match network_id {
            Some(_) => "SELECT COUNT(*) FROM users WHERE is_active = TRUE AND network_id = $1",
            None => "SELECT COUNT(*) FROM users WHERE is_active = TRUE",
        };

        let mut q = sqlx::query_scalar::<_, i64>(query);
        if let Some(nid) = network_id {
            q = q.bind(nid);
        }

        let count = q.fetch_one(&self.pool).await?;
        Ok(count)
    }
}
