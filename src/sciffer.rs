use std::{
    error::Error,
    fmt::{Debug, Display},
};

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

#[derive(Builder)]
pub struct Sciffer<F, E> {
    fetcher: F,
    extracter: E,
}

impl<F, E> Sciffer<F, E>
where
    F: Fetcher,
    E: Extracter,
{
    pub async fn sniffer_sequential<D>(&self) -> Result<Vec<D>, Box<dyn std::error::Error>>
    where
        D: Debug + DeserializeOwned,
    {
        let fetched_data = self
            .fetcher
            .fetch()
            .await
            .map_err(|err| ScifferError::FetcherError(err))?;
        // let extracted_data = fetched_data.into_iter().map(|ctx| async {}).collect();

        let mut res: Vec<D> = vec![];
        for ctx in fetched_data.iter() {
            let topic_data: D = self.extracter.extract(ctx).await?;
            res.push(topic_data);
        }

        Ok(res)
    }

    pub async fn sniffer_parallel<D>(&self) -> Result<Vec<D>, Box<dyn std::error::Error>>
    where
        D: Debug + DeserializeOwned,
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
            futures.push(async move { extracter.extract(&ctx).await });
        }

        let mut res = Vec::new();
        while let Some(result) = futures.next().await {
            res.push(result?);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use langchain_rust::llm::client::Ollama;

    use crate::{
        extracters::topic::{TopicData, TopicExtracterBuilder},
        fetchers::arxiv::ArxivFetcherBuilder,
    };

    use super::ScifferBuilder;

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

        let sciffer = ScifferBuilder::default()
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
