use crate::dao::{KeywordDAO, PaperDAO};
use crate::models::{Keyword, Paper, PaperKeyword};
use sqlx::{query, query_as, SqlitePool};

pub struct PaperKeywordDAO;

impl PaperKeywordDAO {
    pub async fn associate_keyword_with_paper(
        pool: &SqlitePool,
        paper: &Paper,
        keyword: &Keyword,
    ) -> Result<(), sqlx::Error> {
        let pid = PaperDAO::create_paper(pool, paper).await?;
        let kid = KeywordDAO::create_keyword(pool, keyword).await?;
        query!(
            "INSERT INTO paper_keywords (paper_id, keyword_id) VALUES (?, ?)",
            pid,
            kid
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
