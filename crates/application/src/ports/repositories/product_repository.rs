use async_trait::async_trait;

use bio_com_domain::catalog::product::{
    Product,
    ProductId,
};

use super::error::RepositoryError;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: &ProductId,
    ) -> Result<Option<Product>, RepositoryError>;

    async fn find_by_sku(
        &self,
        sku: &str,
    ) -> Result<Option<Product>, RepositoryError>;

    async fn save(
        &self,
        product: &Product,
    ) -> Result<(), RepositoryError>;
}