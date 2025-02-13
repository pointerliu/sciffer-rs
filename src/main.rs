use langchain_rust::llm::client::Ollama;
use sciffer_rs::{extracters::topic::{TopicData, TopicExtracterBuilder}, fetchers::arxiv::ArxivFetcherBuilder, sciffer::ScifferBuilder};

#[tokio::main]
async fn main() {
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

    sciffer.sniffer::<TopicData>().await.unwrap()
        .iter().for_each(|x| println!("{:?}", x));
}
