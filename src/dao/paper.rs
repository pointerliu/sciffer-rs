use crate::models::Paper;
use sqlx::{query, SqlitePool};

pub struct PaperDAO;

impl PaperDAO {
    pub async fn create_paper(pool: &SqlitePool, paper: &Paper) -> Result<i64, sqlx::Error> {
        let existing_paper = query!("SELECT id FROM papers WHERE title = ?", paper.title)
            .fetch_optional(pool)
            .await?;

        if let Some(record) = existing_paper {
            return Ok(record.id);
        }

        let id = query!(
            "INSERT INTO papers (title, abstract_text, publish_date, insert_date, url) VALUES (?, ?, ?, ?, ?) RETURNING id",
            paper.title,
            paper.abstract_text,
            paper.publish_date,
            paper.insert_date,
            paper.url,
        )
            .fetch_one(pool)
            .await?;
        Ok(id.id)
    }
}
