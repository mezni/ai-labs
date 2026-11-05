use sqlx::Transaction;

pub type BoxedTransaction<'a> = Box<dyn sqlx::Transaction<'a, sqlx::Postgres> + Send + 'a>;

pub async fn begin_transaction<'a>(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<BoxedTransaction<'a>, sqlx::Error> {
    let tx: BoxedTransaction<'a> = Box::new(pool.begin().await?);
    Ok(tx)
}