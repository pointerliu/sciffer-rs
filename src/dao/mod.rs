mod keyword;
mod paper;
mod paper_keyword;

use crate::models::{Keyword, Paper};
pub use keyword::*;
pub use paper::*;
pub use paper_keyword::*;
use sqlx::SqlitePool;

pub async fn add_paper_with_keywords(
    pool: &SqlitePool,
    paper: &Paper,
    keywords: &Vec<Keyword>,
) -> Result<(), sqlx::Error> {
    for keyword in keywords.iter() {
        PaperKeywordDAO::associate_keyword_with_paper(pool, paper, keyword).await?;
    }
    Ok(())
}
