use std::collections::HashMap;

pub mod arxiv;

#[derive(Debug)]
pub struct FetcherError {
    msg: String,
}

impl From<String> for FetcherError {
    fn from(value: String) -> Self {
        Self { msg: value }
    }
}

pub trait Fetcher {
    async fn fetch(&self) -> Result<Vec<HashMap<String, String>>, FetcherError>;
}
