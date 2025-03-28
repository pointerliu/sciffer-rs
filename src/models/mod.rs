use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Paper {
    pub id: i64,
    pub title: String,
    pub abstract_text: Option<String>,
    pub publish_date: Option<String>,
    pub insert_date: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Keyword {
    pub id: i64,
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaperKeyword {
    pub paper_id: i32,
    pub keyword_id: i32,
}
