use std::{
    error::Error,
    fmt::{Debug, Display},
};

use arxiv::Arxiv;
use derive_builder::Builder;
use futures::{stream::FuturesUnordered, StreamExt};
use serde::de::DeserializeOwned;

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
    type ExtracterInput;
    fn sniffer_parallel<D>(
        &self,
    ) -> impl std::future::Future<
        Output = Result<Vec<(Self::ExtracterInput, D)>, Box<dyn std::error::Error>>,
    > + Send
    where
        D: Debug + DeserializeOwned + Send;
}

#[derive(Builder)]
pub struct ArxivSciffer<F, E> {
    fetcher: F,
    extracter: E,
}

impl<F, E> Sniffer for ArxivSciffer<F, E>
where
    F: Fetcher<Output = Arxiv> + Sync,
    E: Extracter<Input = Arxiv> + Sync,
{
    type ExtracterInput = E::Input;
    async fn sniffer_parallel<D>(
        &self,
    ) -> Result<Vec<(Self::ExtracterInput, D)>, Box<dyn std::error::Error>>
    where
        D: Debug + DeserializeOwned + Send,
    {
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
                println!("error when processing, {:?}", result);
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use langchain_rust::llm::client::Ollama;

    use crate::sciffer::Sniffer;
    use crate::{
        extracters::topic::{TopicData, TopicExtracterBuilder},
        fetchers::arxiv::ArxivFetcherBuilder,
        sciffer::ArxivScifferBuilder,
    };

    #[tokio::test]
    async fn test_sciffer() {
        let fetcher = ArxivFetcherBuilder::default()
            .query("machine learning".to_string())
            .number(5)
            .build()
            .unwrap();

        let llm = Ollama::default().with_model("llama3.2:3b");
        let extracter = TopicExtracterBuilder::default()
            .llm(Box::new(llm))
            .build()
            .unwrap();

        let sciffer = ArxivScifferBuilder::default()
            .fetcher(fetcher)
            .extracter(extracter)
            .build()
            .unwrap();

        sciffer
            .sniffer_parallel::<TopicData>()
            .await
            .unwrap()
            .iter()
            .for_each(|x| println!("{:?}", x));
    }
}
