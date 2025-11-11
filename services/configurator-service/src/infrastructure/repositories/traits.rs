use crate::core::AppError;
use async_trait::async_trait;

// Generic repository trait for all entities
#[async_trait]
pub trait Repository<T, ID>: Send + Sync {
    async fn create(&self, entity: &T) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, AppError>;
    async fn find_all(&self) -> Result<Vec<T>, AppError>;
    async fn update(&self, entity: &T) -> Result<(), AppError>;
    async fn delete(&self, id: ID) -> Result<(), AppError>;
}

// Pagination support
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl Pagination {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.min(100), // Cap at 100 for performance
        }
    }

    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.per_page
    }
}

// Filtering support
#[derive(Debug, Clone, Default)]
pub struct QueryFilters {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" or "desc"
}

impl QueryFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }

    pub fn with_sort(mut self, sort_by: &str, sort_order: &str) -> Self {
        self.sort_by = Some(sort_by.to_string());
        self.sort_order = Some(sort_order.to_string());
        self
    }
}
