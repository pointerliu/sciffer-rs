use async_openai::config::OPENAI_API_BASE;
use clap::Parser;
use langchain_rust::llm::OpenAI;
use langchain_rust::tools::OpenAIConfig;
use sciffer_rs::{
    analyzers::simple::SimpleArixvTrendingAnalyzerBuilder,
    extracters::topic::TopicExtracterBuilder,
    fetchers::arxiv::ArxivFetcherBuilder,
    sciffer::{ArxivScifferBuilder, Sniffer},
};
use std::env;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    query: String,
    #[arg(short, long)]
    num: i32,
    #[arg(
        short,
        long,
        default_value = "deepseek-ai/DeepSeek-R1-Distill-Llama-7B"
    )]
    model: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let args = Args::parse();

    let fetcher = ArxivFetcherBuilder::default()
        .query(args.query)
        .number(args.num)
        .build()
        .unwrap();

    // let llm = Ollama::default().with_model("llama3.2:3b");
    let llm = OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(env::var("API_BASE").unwrap_or(OPENAI_API_BASE.to_string()))
                .with_api_key(env::var("API_KEY").expect("Are you waiting for my API_KEY?")),
        )
        .with_model(args.model);

    let extracter = TopicExtracterBuilder::default()
        .llm(Box::new(llm))
        .build()
        .unwrap();

    let analyzer = SimpleArixvTrendingAnalyzerBuilder::default()
        .build()
        .unwrap();

    let sciffer = ArxivScifferBuilder::default()
        .fetcher(fetcher)
        .extracter(extracter)
        .analyzer(analyzer)
        .build()
        .unwrap();

    sciffer.sniffer_parallel().await.unwrap();
}
