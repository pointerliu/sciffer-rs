use async_openai::config::OPENAI_API_BASE;
use langchain_rust::llm::OpenAI;
use langchain_rust::tools::OpenAIConfig;
use sciffer_rs::{extracters::topic::{TopicData, TopicExtracterBuilder}, fetchers::arxiv::ArxivFetcherBuilder, sciffer::ScifferBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(5)
        .build()
        .unwrap();

    // let llm = Ollama::default().with_model("llama3.2:3b");
    let llm = OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(env::var("API_BASE").unwrap_or(OPENAI_API_BASE.to_string()))
                .with_api_key(env::var("API_KEY").expect("Are you waiting for my API_KEY?"))
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

    sciffer.sniffer_parallel::<TopicData>().await.unwrap()
        .iter().for_each(|x| println!("{:?}", x));
}
