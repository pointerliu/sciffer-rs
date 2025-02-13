use futures::StreamExt;
use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_template,
    llm::client::Ollama,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};

#[tokio::main]
async fn main() {
    // 1. fetech paper -> title & summary
    // 2. llm prompt -> json
    // 3. parse json -> keywords
    // 4. analysis -> report

    // let open_ai = OpenAI::default();
    let open_ai = Ollama::default().with_model("deepseek-r1:1.5b");

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "You are world class technical documentation writer."
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        )))
    ];

    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai.clone())
        .build()
        .unwrap();

    let mut stream = chain
        .stream(prompt_args! {
        "input" => "What is Code search, What does it can be used for, Why is it effective? summary in short, no more 50 words",
           })
        .await
        .unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(value) => value.to_stdout().unwrap(),
            Err(e) => panic!("Error invoking LLMChain: {:?}", e),
        }
    }
}
