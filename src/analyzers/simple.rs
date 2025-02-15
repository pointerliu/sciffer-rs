use std::collections::HashMap;

use arxiv::Arxiv;
use derive_builder::Builder;

use crate::extracters::topic::TopicData;

use super::TrendingAnalyzer;

#[derive(Builder, Clone)]
pub struct SimpleArixvTrendingAnalyzer {}

impl TrendingAnalyzer for SimpleArixvTrendingAnalyzer {
    type Raw = Arxiv;
    type Ctx = TopicData;

    fn top_k<F: Fn(&Self::Ctx) -> Vec<String>>(&self, data: &Vec<(Self::Raw, Self::Ctx)>, f: F) -> Vec<(String, Vec<Self::Raw>)> {
        let mut cnt: HashMap<String, Vec<Self::Raw>> = HashMap::new();

        for (raw, ctx) in data {
            let problems = f(ctx).clone();
            for problem in problems.iter() {
                let entry = cnt.entry(problem.clone()).or_insert(Vec::new());
                entry.push(raw.clone());
            }
        }

        let mut sorted_problems: Vec<(String, Vec<Self::Raw>)> = cnt
            .into_iter()
            .map(|(problem, raws)| {
                // println!("problem: {} = {}", problem, raws.len());
                (problem, raws)
            })
            .collect();

        sorted_problems.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(b.0.cmp(&a.0))); // Sort by the length of associated raws (i.e., count)

        sorted_problems
    }
}
