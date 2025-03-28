use std::{
    error::Error,
    fmt::{Debug, Display},
};

use arxiv::Arxiv;
use derive_builder::Builder;
use futures::{stream::FuturesUnordered, StreamExt};

use crate::extracters::topic::ArxivTopicData;
use crate::{
    extracters::Extracter,
    fetchers::{Fetcher, FetcherError},
};

#[derive(Debug)]
enum ScifferError {
    FetcherError(FetcherError),
    #[allow(dead_code)]
    ExtracterError,
}

impl Display for ScifferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FetcherError(err) => write!(f, "{:?}", err),
            Self::ExtracterError => write!(f, "extracter error"),
        }
    }
}
impl Error for ScifferError {}

pub trait Sniffer {
    type Input;
    type Output;
    fn sniffer_parallel(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<(Self::Input, Self::Output)>, Box<dyn Error>>> + Send;
}

#[derive(Builder)]
pub struct ArxivSciffer<F, E> {
    fetcher: F,
    extracter: E,
}

impl<F, E> Sniffer for ArxivSciffer<F, E>
where
    F: Fetcher<Output = Arxiv> + Sync,
    E: Extracter<Input = Arxiv, Output = ArxivTopicData> + Sync,
{
    type Input = Arxiv;
    type Output = ArxivTopicData;
    async fn sniffer_parallel(&self) -> Result<Vec<(Self::Input, Self::Output)>, Box<dyn Error>> {
        let fetched_data = self
            .fetcher
            .fetch()
            .await
            .map_err(|err| ScifferError::FetcherError(err))?;
        // let extracted_data = fetched_data.into_iter().map(|ctx| async {}).collect();

        let mut futures = FuturesUnordered::new();
        for ctx in fetched_data.iter() {
            let extracter = &self.extracter;
            futures.push(async move { (ctx.clone(), extracter.extract(&ctx).await) });
        }

        let mut res = Vec::new();
        while let Some(result) = futures.next().await {
            if let Ok(d) = result.1 {
                res.push((result.0, d));
            } else {
                println!(
                    "error when processing, arxiv id: {:?} with err {:?}",
                    result.0.id, result.1
                );
            }
        }

        Ok(res)
    }
}
