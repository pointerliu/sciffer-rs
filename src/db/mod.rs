use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn get_db_pool() -> Result<SqlitePool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite://my_database.db".into());
    SqlitePool::connect(&db_url).await
}
