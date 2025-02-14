pub mod arxiv;

#[derive(Debug)]
pub struct FetcherError {
    #[allow(dead_code)]
    msg: String,
}

impl From<String> for FetcherError {
    fn from(value: String) -> Self {
        Self { msg: value }
    }
}

pub trait Fetcher {
    type Output;

    fn fetch(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Output>, FetcherError>> + Send;
}
