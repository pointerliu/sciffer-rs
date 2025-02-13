use std::collections::HashMap;

use arxiv::{ArxivQuery, ArxivQueryBuilder};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use super::{Fetcher, FetcherError};

#[derive(Builder, Clone)]
pub struct ArxivFetcher {
    // #[builder(setter(custom))]
    query: String,
    number: i32,
}

impl Default for ArxivFetcher {
    fn default() -> Self {
        Self {
            query: "Are you querying me?".to_string(),
            number: 5,
        }
    }
}

impl Fetcher for ArxivFetcher {
    async fn fetch(&self) -> Result<Vec<HashMap<String, String>>, FetcherError> {
        let arxiv_query = self.build_arixv_query();
        let arxivs = arxiv::fetch_arxivs(arxiv_query)
            .await
            .map_err(|e| format!("arixv::fetch_arxivs error {:?}", e))?;

        Ok(arxivs
            .into_iter()
            .map(|x| {
                HashMap::from([
                    ("title".to_string(), x.title),
                    ("summary".to_string(), x.summary),
                ])
            })
            .collect())
    }
}

impl ArxivFetcher {
    fn build_arixv_query(&self) -> ArxivQuery {
        ArxivQueryBuilder::new()
            .search_query(&self.query)
            .start(0)
            .max_results(self.number)
            .sort_by("submittedDate")
            .sort_order("descending")
            .build()
    }
}

// impl ArxivFetcherBuilder {
//     pub fn query(mut self, query: &str) -> Self {
//         if self.query.is_some() {
//             self.query.as_mut().unwrap().push(query.to_string());
//         } else {
//             self.query = Some(vec![query.to_string()]);
//         };
//         self
//     }
// }

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::fetchers::Fetcher;

    use super::{ArxivFetcher, ArxivFetcherBuilder};

    #[tokio::test]
    async fn test_arxiv_fetcher() {
        let fetcher = ArxivFetcherBuilder::default()
            .query("program+AND+repair+OR+generation+OR+verification".to_string())
            .number(10)
            .build()
            .unwrap();
        let res = fetcher.fetch().await.unwrap();

        res.iter().for_each(|x| println!("title = {:?}", x));
    }
}
