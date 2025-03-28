use crate::models::Keyword;
use sqlx::{query, query_as, SqlitePool};

pub struct KeywordDAO;

impl KeywordDAO {
    pub async fn create_keyword(pool: &SqlitePool, keyword: &Keyword) -> Result<i64, sqlx::Error> {
        let id = query!(
            "INSERT INTO keywords (keyword) VALUES (?) RETURNING id",
            keyword.keyword,
        )
        .fetch_one(pool)
        .await?;
        Ok(id.id)
    }

    // Retrieve all papers
    pub async fn get_all_papers(pool: &SqlitePool) -> Result<Vec<Keyword>, sqlx::Error> {
        let papers = query_as!(Keyword, "SELECT * FROM keywords")
            .fetch_all(pool)
            .await?;
        Ok(papers)
    }

    // Retrieve a paper by ID
    pub async fn get_paper_by_id(
        pool: &SqlitePool,
        id: i32,
    ) -> Result<Option<Keyword>, sqlx::Error> {
        let paper = query_as!(Keyword, "SELECT * FROM keywords WHERE id = ?", id)
            .fetch_optional(pool)
            .await?;
        Ok(paper)
    }

    // Delete a paper by ID
    pub async fn delete_paper(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        query!("DELETE FROM keywords WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
