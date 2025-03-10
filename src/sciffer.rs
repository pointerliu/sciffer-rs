use std::{
    error::Error,
    fmt::{Debug, Display},
};

use arxiv::Arxiv;
use derive_builder::Builder;
use futures::{stream::FuturesUnordered, StreamExt};
use serde::de::DeserializeOwned;

use crate::{
    analyzers::TrendingAnalyzer,
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

pub trait Sniffer<D> {
    type ExtracterInput;
    fn sniffer_parallel<M: Fn(&D) -> Vec<String> + Send>(
        &self,
        f: M,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}

#[derive(Builder)]
pub struct ArxivSciffer<F, E, A> {
    fetcher: F,
    extracter: E,
    analyzer: A,
}

impl<F, E, A, D> Sniffer<D> for ArxivSciffer<F, E, A>
where
    F: Fetcher<Output = Arxiv> + Sync,
    E: Extracter<Input = F::Output> + Sync,
    A: TrendingAnalyzer<Raw = F::Output, Ctx = D> + Sync,
    D: Debug + DeserializeOwned + Send,
{
    type ExtracterInput = E::Input;
    async fn sniffer_parallel<M: Fn(&D) -> Vec<String> + Send>(
        &self,
        f: M,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fetched_data = self
            .fetcher
            .fetch()
            .await
            .map_err(|err| ScifferError::FetcherError(err))?;
        // let extracted_data = fetched_data.into_iter().map(|ctx| async {}).collect();

        let mut futures = FuturesUnordered::new();
        for ctx in fetched_data.iter() {
            let extracter = &self.extracter;
            futures.push(async move { (ctx.clone(), extracter.extract::<D>(&ctx).await) });
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

        let trending_res = self.analyzer.top_k(&res, f);

        println!("{:#?}", trending_res);
        for (i, (keyword, papers)) in trending_res.iter().enumerate() {
            println!("HOT {}: {}", i, keyword);
            println!("[");
            for p in papers {
                println!("\tid={}", p.id);
                println!("\ttitle={}", p.title);
            }
            println!("]");
        }

        Ok(())
    }
}
