// src/application/user_queries.rs
use crate::application::user_dtos::*;
use crate::core::{AppError, constants::*, errors::AppResult};
use crate::domain::repositories::{SortOrder, UserFilters, UserRepository};
use std::sync::Arc;

pub struct UserQueries {
    user_repo: Arc<dyn UserRepository>,
}

impl UserQueries {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse::from(user))
    }

    /// Get user detail (with extra info)
    pub async fn get_user_detail(&self, user_id: &str) -> AppResult<UserDetailResponse> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserDetailResponse::from_user(user))
    }

    /// Get current user profile
    pub async fn get_my_profile(&self, user_id: &str) -> AppResult<UserResponse> {
        self.get_user(user_id).await
    }

    /// List users with filters and pagination
    pub async fn list_users(&self, request: ListUsersRequest) -> AppResult<PaginatedUsersResponse> {
        // Parse sort order
        let sort_order = match request.sort_order.as_deref() {
            Some("asc") => Some(SortOrder::Asc),
            Some("desc") => Some(SortOrder::Desc),
            _ => None,
        };

        // Build filters
        let filters = UserFilters {
            search: request.search,
            role: request.role,
            source: request.source,
            network_id: request.network_id,
            station_id: request.station_id,
            is_active: request.is_active,
            is_verified: request.is_verified,
            include_deleted: request.include_deleted,
            page: Some(request.page),
            page_size: Some(request.page_size),
            sort_by: request.sort_by,
            sort_order,
        };

        // Get users and total count
        let users = self.user_repo.list(filters.clone()).await?;
        let total = self.user_repo.count(filters).await?;

        let total_pages = (total as f64 / request.page_size as f64).ceil() as i64;

        Ok(PaginatedUsersResponse {
            users: users.into_iter().map(UserResponse::from).collect(),
            total,
            page: request.page,
            page_size: request.page_size,
            total_pages,
        })
    }

    /// Get users by role
    pub async fn get_users_by_role(&self, role: &str) -> AppResult<Vec<UserResponse>> {
        if !is_valid_role(role) {
            return Err(AppError::Validation(format!("Invalid role: {}", role)));
        }

        let users = self.user_repo.find_by_role(role).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Get users by network
    pub async fn get_users_by_network(&self, network_id: &str) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.find_by_network_id(network_id).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Get users by station
    pub async fn get_users_by_station(&self, station_id: &str) -> AppResult<Vec<UserResponse>> {
        let users = self.user_repo.find_by_station_id(station_id).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        self.user_repo.email_exists(email).await
    }

    /// Check if username exists
    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        self.user_repo.username_exists(username).await
    }
}
