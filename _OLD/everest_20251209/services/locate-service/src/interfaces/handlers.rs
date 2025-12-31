use crate::{
    application::{dto::*, review_service::ReviewService, station_service::StationService},
    infrastructure::error::{AppError, AppResult},
    middleware::extract_claims,
};
use actix_web::{HttpRequest, HttpResponse, web};

/// Get nearby stations based on location
#[utoipa::path(
    get,
    path = "/api/v1/stations/nearby",
    params(
        ("latitude" = f64, Query, description = "Latitude coordinate"),
        ("longitude" = f64, Query, description = "Longitude coordinate"),
        ("radius_meters" = Option<i32>, Query, description = "Search radius in meters (default: 5000)"),
        ("limit" = Option<i32>, Query, description = "Maximum number of results (default: 50)")
    ),
    responses(
        (status = 200, description = "List of nearby stations", body = Vec<StationResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "stations"
)]
pub async fn get_nearby_stations(
    query: web::Query<NearbyStationsQuery>,
    station_service: web::Data<StationService>,
) -> AppResult<HttpResponse> {
    let stations = station_service
        .find_nearby_stations(
            query.latitude,
            query.longitude,
            query.radius_meters,
            query.limit,
        )
        .await?;

    Ok(HttpResponse::Ok().json(stations))
}

/// Create a new review
#[utoipa::path(
    post,
    path = "/api/v1/reviews",
    request_body = CreateReviewRequest,
    responses(
        (status = 201, description = "Review created successfully", body = ReviewResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - user role required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "reviews"
)]
pub async fn create_review(
    req: HttpRequest,
    payload: web::Json<CreateReviewRequest>,
    review_service: web::Data<ReviewService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    // Validate rating range
    if payload.rating < 1 || payload.rating > 5 {
        return Err(AppError::ValidationError(
            "Rating must be between 1 and 5".to_string(),
        ));
    }

    // Extract user_id from claims (you might want to add this to Claims struct)
    let user_id = claims.jti.clone(); // Using jti as user_id, adjust as needed

    let review = review_service
        .create_review(
            user_id.clone(),
            payload.station_id.clone(),
            payload.rating,
            payload.review_text.clone(),
            user_id.clone(),
        )
        .await?;

    Ok(HttpResponse::Created().json(review))
}

/// Get reviews for a specific station
#[utoipa::path(
    get,
    path = "/api/v1/reviews/station/{station_id}",
    params(
        ("station_id" = String, Path, description = "Station ID")
    ),
    responses(
        (status = 200, description = "List of reviews for the station", body = Vec<ReviewResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - user role required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "reviews"
)]
pub async fn get_station_reviews(
    req: HttpRequest,
    station_id: web::Path<String>,
    review_service: web::Data<ReviewService>,
) -> AppResult<HttpResponse> {
    let _claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    let reviews = review_service.get_reviews_by_station(&station_id).await?;

    Ok(HttpResponse::Ok().json(reviews))
}

/// Update a review
#[utoipa::path(
    put,
    path = "/api/v1/reviews/{review_id}",
    params(
        ("review_id" = String, Path, description = "Review ID")
    ),
    request_body = UpdateReviewRequest,
    responses(
        (status = 200, description = "Review updated successfully", body = ReviewResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - user role required"),
        (status = 404, description = "Review not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "reviews"
)]
pub async fn update_review(
    req: HttpRequest,
    review_id: web::Path<String>,
    payload: web::Json<UpdateReviewRequest>,
    review_service: web::Data<ReviewService>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    if let Some(rating) = payload.rating {
        if rating < 1 || rating > 5 {
            return Err(AppError::ValidationError(
                "Rating must be between 1 and 5".to_string(),
            ));
        }
    }

    let updated_by = claims.jti.clone();

    let review = review_service
        .update_review(
            &review_id,
            payload.rating,
            payload.review_text.clone(),
            updated_by,
        )
        .await?;

    Ok(HttpResponse::Ok().json(review))
}

/// Delete a review
#[utoipa::path(
    delete,
    path = "/api/v1/reviews/{review_id}",
    params(
        ("review_id" = String, Path, description = "Review ID")
    ),
    responses(
        (status = 204, description = "Review deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - user role required"),
        (status = 404, description = "Review not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "reviews"
)]
pub async fn delete_review(
    req: HttpRequest,
    review_id: web::Path<String>,
    review_service: web::Data<ReviewService>,
) -> AppResult<HttpResponse> {
    let _claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    review_service.delete_review(&review_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "health"
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "locate-service"
    }))
}

/// Get user info from token
#[utoipa::path(
    get,
    path = "/api/v1/user/info",
    responses(
        (status = 200, description = "User information from token"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "user"
)]
pub async fn get_user_info(req: HttpRequest) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req)
        .ok_or_else(|| AppError::Unauthorized("Missing authentication".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user_id": claims.jti,
        "roles": claims.roles,
        "network_id": claims.network_id,
        "station_id": claims.station_id,
    })))
}
