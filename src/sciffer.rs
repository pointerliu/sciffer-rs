use crate::dao::add_paper_with_keywords;
use crate::extracters::topic::ArxivTopicData;
use crate::models::{Keyword, Paper};
use crate::{
    db,
    extracters::Extracter,
    fetchers::{Fetcher, FetcherError},
};
use arxiv::Arxiv;
use async_trait::async_trait;
use chrono::Local;
use derive_builder::Builder;
use futures::{stream::FuturesUnordered, StreamExt};
use std::time::Duration;
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use tokio::time::sleep;

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
    ) -> impl std::future::Future<
        Output = Result<Vec<(Self::Input, Self::Output)>, Box<dyn std::error::Error + Send + Sync>>,
    > + Send;
}

#[async_trait]
pub trait SnifferServer: Sniffer {
    async fn start_server(&self) -> Result<(), sqlx::Error>;
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
    async fn sniffer_parallel(
        &self,
    ) -> Result<Vec<(Self::Input, Self::Output)>, Box<dyn Error + Send + Sync>> {
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

#[async_trait]
impl<F, E> SnifferServer for ArxivSciffer<F, E>
where
    F: Fetcher<Output = Arxiv> + Sync,
    E: Extracter<Input = Arxiv, Output = ArxivTopicData> + Sync,
{
    async fn start_server(&self) -> Result<(), sqlx::Error> {
        let pool = db::get_db_pool().await?;
        loop {
            if let Ok(res) = self.sniffer_parallel().await {
                let today = Local::now();
                let date_str = today.to_string();
                let data = res
                    .iter()
                    .map(|(meta, ext_meta)| {
                        let meta = meta.clone();
                        let ext_meta = ext_meta.clone();
                        let paper = Paper {
                            id: 0,
                            title: meta.title,
                            abstract_text: Some(meta.summary),
                            publish_date: Some(meta.published),
                            insert_date: date_str.clone(),
                            url: meta.pdf_url,
                        };

                        let keywords: Vec<Keyword> = ext_meta
                            .techniques_used
                            .iter()
                            .map(|s| Keyword {
                                id: 0,
                                keyword: s.to_string(),
                            })
                            .collect();
                        (paper, keywords)
                    })
                    .collect::<Vec<_>>();

                for (paper, keywords) in data {
                    if let Err(err) = add_paper_with_keywords(&pool, &paper, &keywords).await {
                        println!("add_paper_with_keywords error: {:?}", err);
                    }
                }

                println!("add papers finished @ {}: {:?}", date_str, res);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
