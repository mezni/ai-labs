use async_trait::async_trait;

use bio_com_domain::order::{
    Order,
    OrderId,
};

use super::error::RepositoryError;

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &OrderId,
    ) -> Result<Option<Order>, RepositoryError>;

    async fn save(
        &self,
        order: &Order,
    ) -> Result<(), RepositoryError>;
}