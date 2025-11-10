use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, ID> {
    type Error;
    
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, Self::Error>;
    async fn save(&self, entity: &T) -> Result<T, Self::Error>;
    async fn update(&self, entity: &T) -> Result<T, Self::Error>;
    async fn delete(&self, id: ID) -> Result<(), Self::Error>;
}