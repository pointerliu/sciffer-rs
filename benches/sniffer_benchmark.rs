use std::{env, fmt::Debug};

use async_openai::config::OPENAI_API_BASE;
use criterion::{criterion_group, criterion_main, Criterion};
use langchain_rust::llm::{OpenAI, OpenAIConfig};
use sciffer_rs::{
    extracters::{
        topic::{TopicData, TopicExtracter, TopicExtracterBuilder},
        Extracter,
    },
    fetchers::{
        arxiv::{ArxivFetcher, ArxivFetcherBuilder},
        Fetcher,
    },
    sciffer::{Sciffer, ScifferBuilder},
};
use serde::de::DeserializeOwned;

use tokio::runtime::Runtime;

fn setup_sciffer() -> Sciffer<ArxivFetcher, TopicExtracter> {
    let _ = dotenv::dotenv();
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(3)
        .build()
        .unwrap();

    // let llm = Ollama::default().with_model("llama3.2:3b");
    let llm = OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(env::var("API_BASE").unwrap_or(OPENAI_API_BASE.to_string()))
                .with_api_key(env::var("API_KEY").expect("Are you waiting for my API_KEY?")),
        )
        .with_model("deepseek-ai/DeepSeek-R1-Distill-Qwen-7B");
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
}

async fn sciffer_parallel<F, E, D>(sciffer: &Sciffer<F, E>)
where
    F: Fetcher,
    E: Extracter,
    D: Debug + DeserializeOwned,
{
    sciffer
        .sniffer_parallel::<D>()
        .await
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{:?}", x)
        });
}

async fn sciffer_sequential<F, E, D>(sciffer: &Sciffer<F, E>)
where
    F: Fetcher,
    E: Extracter,
    D: Debug + DeserializeOwned,
{
    sciffer
        .sniffer_sequential::<D>()
        .await
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{:?}", x)
        });
}

fn benchmark_sniffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("sciffer-process");
    group.sample_size(10);
    
    let sciffer = setup_sciffer();

    let runtime = Runtime::new().unwrap();
    group.bench_function("sniffer_parallel", |b| {
        b.iter(|| {
            runtime.block_on(sciffer_parallel::<_, _, TopicData>(&sciffer));
        })
    });
    group.bench_function("sniffer_sequential", |b| {
        b.iter(|| {
            runtime.block_on(sciffer_sequential::<_, _, TopicData>(&sciffer));
        })
    });
}

criterion_group!(benches, benchmark_sniffer);
criterion_main!(benches);
