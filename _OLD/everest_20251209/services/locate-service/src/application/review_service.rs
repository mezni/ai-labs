use crate::{
    application::dto::ReviewResponse,
    domain::repositories::ReviewRepositoryTrait,
    infrastructure::{ReviewRepository, error::AppResult},
    utils::id_generator::generate_review_id,
};

pub struct ReviewService {
    repository: ReviewRepository,
}

impl ReviewService {
    pub fn new(repository: ReviewRepository) -> Self {
        Self { repository }
    }

    pub async fn create_review(
        &self,
        user_id: String,
        station_id: String,
        rating: i32,
        review_text: Option<String>,
        created_by: String,
    ) -> AppResult<ReviewResponse> {
        let review_id = generate_review_id();

        let review = self
            .repository
            .create(
                review_id,
                user_id,
                station_id,
                rating,
                review_text,
                created_by,
            )
            .await?;

        Ok(ReviewResponse {
            review_id: review.review_id,
            user_id: review.user_id,
            station_id: review.station_id,
            rating: review.rating,
            review_text: review.review_text,
            created_at: review.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: review.updated_at.map(|dt| dt.to_rfc3339()),
        })
    }

    pub async fn get_reviews_by_station(&self, station_id: &str) -> AppResult<Vec<ReviewResponse>> {
        let reviews = self.repository.find_by_station(station_id).await?;

        let response: Vec<ReviewResponse> = reviews
            .into_iter()
            .map(|r| ReviewResponse {
                review_id: r.review_id,
                user_id: r.user_id,
                station_id: r.station_id,
                rating: r.rating,
                review_text: r.review_text,
                created_at: r.created_at.map(|dt| dt.to_rfc3339()),
                updated_at: r.updated_at.map(|dt| dt.to_rfc3339()),
            })
            .collect();

        Ok(response)
    }

    pub async fn update_review(
        &self,
        review_id: &str,
        rating: Option<i32>,
        review_text: Option<String>,
        updated_by: String,
    ) -> AppResult<ReviewResponse> {
        let review = self
            .repository
            .update(review_id, rating, review_text, updated_by)
            .await?;

        Ok(ReviewResponse {
            review_id: review.review_id,
            user_id: review.user_id,
            station_id: review.station_id,
            rating: review.rating,
            review_text: review.review_text,
            created_at: review.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: review.updated_at.map(|dt| dt.to_rfc3339()),
        })
    }

    pub async fn delete_review(&self, review_id: &str) -> AppResult<()> {
        self.repository.delete(review_id).await?;
        Ok(())
    }
}
