use bio_com_application::ports::order_repository::OrderRepository;
use bio_com_domain::orders::Order;
use sqlx::FromRow;

#[derive(FromRow)]
struct OrderRow {
    id: String,
    customer_id: String,
    total: i32,
    status: String,
    created_at: time::OffsetDateTime,
}

pub struct PostgresOrderRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresOrderRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}

impl OrderRepository for PostgresOrderRepository {
    async fn find_by_id(
        &self,
        id: bio_com_domain::orders::OrderId,
    ) -> Result<Option<Order>, bio_com_application::ApplicationError> {
        let row = sqlx::query_as::<_, OrderRow>(
            "SELECT id, customer_id, total, status, created_at FROM orders WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(row.map(|r| Order {
            id: r.id.into(),
            customer_id: r.customer_id.into(),
            total: r.total,
            status: r.status,
            created_at: r.created_at,
        }))
    }

    async fn save(
        &self,
        order: &Order,
    ) -> Result<(), bio_com_application::ApplicationError> {
        sqlx::query(
            "INSERT INTO orders (id, customer_id, total, status, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(order.id.as_str())
        .bind(&order.customer_id)
        .bind(order.total)
        .bind(&order.status)
        .bind(&order.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(())
    }
}