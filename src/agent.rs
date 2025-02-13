use langchain_rust::{
    chain::{Chain, ChainError, LLMChainBuilder},
    language_models::llm::LLM,
    prompt::{FormatPrompter, PromptArgs},
};

pub trait AgentApp {
    fn get_prompt(&self) -> Box<dyn FormatPrompter>;
    fn get_llm(&self) -> Box<dyn LLM>;
    fn invoke(&self, args: PromptArgs) -> impl std::future::Future<Output = Result<String, ChainError>> {
        let llm = self.get_llm();

        let prompt = self.get_prompt();

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(llm)
            .build()
            .unwrap();
        
        async move {
            chain.invoke(args).await.map_err(|e| e.into())
        }
    }
}
