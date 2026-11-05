use bio_com_application::ports::product_repository::ProductRepository;
use bio_com_domain::products::Product;
use sqlx::FromRow;

#[derive(FromRow)]
struct ProductRow {
    id: String,
    name: String,
    price: i32,
    created_at: time::OffsetDateTime,
}

pub struct PostgresProductRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresProductRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}

impl ProductRepository for PostgresProductRepository {
    async fn find_by_id(
        &self,
        id: bio_com_domain::products::ProductId,
    ) -> Result<Option<Product>, bio_com_application::ApplicationError> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, name, price, created_at FROM products WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(row.map(|r| Product {
            id: r.id.into(),
            name: r.name,
            price: r.price,
            created_at: r.created_at,
        }))
    }

    async fn save(
        &self,
        product: &Product,
    ) -> Result<(), bio_com_application::ApplicationError> {
        sqlx::query(
            "INSERT INTO products (id, name, price, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(product.id.as_str())
        .bind(&product.name)
        .bind(product.price)
        .bind(&product.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(())
    }
}