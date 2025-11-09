use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, Box<dyn std::error::Error>>;
    async fn save(&self, entity: T) -> Result<T, Box<dyn std::error::Error>>;
    async fn delete(&self, id: ID) -> Result<bool, Box<dyn std::error::Error>>;
}
