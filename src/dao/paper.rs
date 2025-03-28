use crate::models::paper::Paper;
use sqlx::{query, query_as, SqlitePool};

pub struct PaperDAO;

impl PaperDAO {
    pub async fn create_paper(pool: &SqlitePool, paper: &Paper) -> Result<(), sqlx::Error> {
        query!(
            "INSERT INTO papers (title, abstract_text, publish_date, insert_date, url, keywords) VALUES (?, ?, ?, ?, ?, ?)",
            paper.title,
            paper.abstract_text,
            paper.publish_date,
            paper.insert_date,
            paper.url,
            paper.keywords
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    // Retrieve all papers
    pub async fn get_all_papers(pool: &SqlitePool) -> Result<Vec<Paper>, sqlx::Error> {
        let papers = query_as!(Paper, "SELECT * FROM papers")
            .fetch_all(pool)
            .await?;
        Ok(papers)
    }

    // Retrieve a paper by ID
    pub async fn get_paper_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Paper>, sqlx::Error> {
        let paper = query_as!(Paper, "SELECT * FROM papers WHERE id = ?", id)
            .fetch_optional(pool)
            .await?;
        Ok(paper)
    }

    // Delete a paper by ID
    pub async fn delete_paper(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        query!("DELETE FROM papers WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
