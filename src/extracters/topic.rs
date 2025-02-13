use std::{collections::HashMap, fmt::Debug, fs};

use derive_builder::Builder;
use langchain_rust::{
    fmt_template,
    language_models::llm::LLM,
    llm::client::Ollama,
    message_formatter,
    prompt::{FormatPrompter, HumanMessagePromptTemplate},
    prompt_args, template_fstring,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{agent::AgentApp, tools::parser::parse_json_md};

use super::{Extracter, ExtracterError};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct TopicExtracter {
    llm: Box<dyn LLM>,
}

impl Clone for TopicExtracter {
    fn clone(&self) -> Self {
        Self {
            llm: self.llm.clone_box(),
        }
    }
}

impl Default for TopicExtracter {
    fn default() -> Self {
        let llm = Ollama::default().with_model("llama3.2:3b");
        Self { llm: Box::new(llm) }
    }
}

impl Extracter for TopicExtracter {
    async fn extract<D>(&self, ctx: &HashMap<String, String>) -> Result<D, ExtracterError>
    where
        D: DeserializeOwned + Debug,
    {
        let args = prompt_args![
            "title" => ctx["title"],
            "summary" => ctx["summary"]];
        let data = self
            .invoke(args)
            .await
            .map_err(|err| ExtracterError::ChainError(err, format!("{:?}", ctx)))?;
        let json_data = parse_json_md(&data)
            .map_err(|err| ExtracterError::Other(err, format!("{:?}", data)))?;

        Ok(serde_json::from_value(json_data.clone())
            .map_err(|err| ExtracterError::Other(Box::new(err), format!("{:?}", json_data)))?)
    }
}

impl AgentApp for TopicExtracter {
    fn get_prompt(&self) -> Box<dyn FormatPrompter> {
        let topic_prompt = fs::read_to_string("prompts/topic.pt").unwrap();
        let prompt = message_formatter![fmt_template!(HumanMessagePromptTemplate::new(
            template_fstring!(topic_prompt, "title", "summary")
        ))];
        Box::new(prompt)
    }

    fn get_llm(&self) -> Box<dyn LLM> {
        self.llm.clone_box()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicData {
    title: String,
    solved_problem: Vec<String>,
    research_field: Vec<String>,
    techniques_used: Vec<String>,
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use langchain_rust::llm::client::Ollama;

    use crate::extracters::{
        topic::{TopicData, TopicExtracterBuilder},
        Extracter,
    };

    #[tokio::test]
    async fn test_arxiv_fetcher() {
        let ollama = Ollama::default().with_model("llama3.2:3b");
        let extracter = TopicExtracterBuilder::default()
            .llm(Box::new(ollama))
            .build()
            .unwrap();

        let mut ctx = HashMap::new();
        ctx.insert(
            "title".to_string(),
            "Deep Residual Learning for Image Recognition".to_string(),
        );
        ctx.insert("summary".to_string(), r#"
            Deeper neural networks are more difficult to train. We
            present a residual learning framework to ease the training
            of networks that are substantially deeper than those used
            previously. We explicitly reformulate the layers as learning residual functions with reference to the layer inputs, instead of learning unreferenced functions. We provide comprehensive empirical evidence showing that these residual
            networks are easier to optimize, and can gain accuracy from
            considerably increased depth. On the ImageNet dataset we
            evaluate residual nets with a depth of up to 152 layers—8×
            deeper than VGG nets [40] but still having lower complexity. An ensemble of these residual nets achieves 3.57% error
            on the ImageNet test set. This result won the 1st place on the
            ILSVRC 2015 classification task. We also present analysis
            on CIFAR-10 with 100 and 1000 layers.
            The depth of representations is of central importance
            for many visual recognition tasks. Solely due to our extremely deep representations, we obtain a 28% relative improvement on the COCO object detection dataset. Deep
            residual nets are foundations of our submissions to ILSVRC
            & COCO 2015 competitions1
            , where we also won the 1st
            places on the tasks of ImageNet detection, ImageNet localization, COCO detection, and COCO segmentation
            "#.to_string());
        let res: TopicData = extracter.extract(&ctx).await.unwrap();

        println!("{:?}", res)
    }
}
