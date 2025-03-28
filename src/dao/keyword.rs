use crate::models::Keyword;
use sqlx::{query, SqlitePool};

pub struct KeywordDAO;

impl KeywordDAO {
    pub async fn create_keyword(pool: &SqlitePool, keyword: &Keyword) -> Result<i64, sqlx::Error> {
        let existing_keyword = query!("SELECT id FROM keywords WHERE keyword = ?", keyword.keyword)
            .fetch_optional(pool)
            .await?;

        if let Some(record) = existing_keyword {
            return Ok(record.id);
        }

        let inserted_id = query!(
            "INSERT INTO keywords (keyword) VALUES (?) RETURNING id",
            keyword.keyword,
        )
        .fetch_one(pool)
        .await?;

        Ok(inserted_id.id)
    }
}
