use async_trait::async_trait;

use bio_com_domain::customer::{
    Customer,
    CustomerId,
};

use super::error::RepositoryError;

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &CustomerId,
    ) -> Result<Option<Customer>, RepositoryError>;

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<Customer>, RepositoryError>;

    async fn save(
        &self,
        customer: &Customer,
    ) -> Result<(), RepositoryError>;
}