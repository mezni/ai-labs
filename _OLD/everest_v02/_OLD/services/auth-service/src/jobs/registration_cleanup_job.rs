use crate::infrastructure::repositories_pg::{PgRegistrationRepository, RegistrationRepository};
use sqlx::PgPool;
use tokio::time::{interval, Duration};

pub async fn run_cleanup_job(pool: PgPool) {
    let mut tick = interval(Duration::from_secs(3600)); // Run every hour

    loop {
        tick.tick().await;

        tracing::info!("Running registration cleanup job");

        let repo = PgRegistrationRepository::new(pool.clone());

        match cleanup_expired_registrations(&repo).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Cleaned up {} expired registrations", count);
                }
            }
            Err(e) => {
                tracing::error!("Cleanup job failed: {}", e);
            }
        }
    }
}

async fn cleanup_expired_registrations<R: RegistrationRepository>(
    repo: &R,
) -> Result<usize, Box<dyn std::error::Error>> {
    let expired = repo.find_expired().await?;
    let count = expired.len();

    if count > 0 {
        let ids: Vec<String> = expired.into_iter().map(|r| r.registration_id).collect();

        repo.mark_expired(ids).await?;
    }

    Ok(count)
}
