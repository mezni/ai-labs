use bio_com_application::ports::customer_repository::CustomerRepository;
use bio_com_domain::customers::Customer;
use sqlx::FromRow;

#[derive(FromRow)]
struct CustomerRow {
    id: String,
    name: String,
    email: String,
    created_at: time::OffsetDateTime,
}

pub struct PostgresCustomerRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresCustomerRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}

impl CustomerRepository for PostgresCustomerRepository {
    async fn find_by_id(
        &self,
        id: bio_com_domain::customers::CustomerId,
    ) -> Result<Option<Customer>, bio_com_application::ApplicationError> {
        let row = sqlx::query_as::<_, CustomerRow>(
            "SELECT id, name, email, created_at FROM customers WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(row.map(|r| Customer {
            id: r.id.into(),
            name: r.name,
            email: r.email,
            created_at: r.created_at,
        }))
    }

    async fn save(
        &self,
        customer: &Customer,
    ) -> Result<(), bio_com_application::ApplicationError> {
        sqlx::query(
            "INSERT INTO customers (id, name, email, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(customer.id.as_str())
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| bio_com_application::ApplicationError::Repository(e.to_string()))?;

        Ok(())
    }
}