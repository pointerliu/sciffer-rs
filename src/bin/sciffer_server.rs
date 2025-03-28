use async_openai::config::OPENAI_API_BASE;
use clap::Parser;
use langchain_rust::language_models::llm::LLM;
use langchain_rust::llm::{client::Ollama, OpenAI};
use langchain_rust::tools::OpenAIConfig;
use sciffer_rs::sciffer::SnifferServer;
use sciffer_rs::{
    extracters::topic::TopicExtracterBuilder, fetchers::arxiv::ArxivFetcherBuilder,
    sciffer::ArxivScifferBuilder,
};
use std::env;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    query: String,
    #[arg(short, long)]
    num: i32,
    #[arg(short, long, default_value_t = false)]
    ollama: bool,
    #[arg(
        short,
        long,
        default_value = "deepseek-ai/DeepSeek-R1-Distill-Llama-8B"
    )]
    model: String,
    #[arg(short, long, default_value = "prompts/topic.pt")]
    prompt: String,
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

    let llm: Box<dyn LLM> = if args.ollama {
        Box::new(Ollama::default().with_model(args.model))
    } else {
        Box::new(
            OpenAI::default()
                .with_config(
                    OpenAIConfig::default()
                        .with_api_base(env::var("API_BASE").unwrap_or(OPENAI_API_BASE.to_string()))
                        .with_api_key(
                            env::var("API_KEY").expect("Are you waiting for my API_KEY?"),
                        ),
                )
                .with_model(args.model),
        )
    };

    let extracter = TopicExtracterBuilder::default()
        .prompt(args.prompt)
        .llm(llm)
        .build()
        .unwrap();

    let sciffer = ArxivScifferBuilder::default()
        .fetcher(fetcher)
        .extracter(extracter)
        .build()
        .unwrap();

    sciffer.start_server().await.unwrap();
}
