use std::{
    error::Error,
    fmt::{Debug, Display},
};

use langchain_rust::chain::ChainError;
use serde::de::DeserializeOwned;

pub mod topic;

#[derive(Debug)]
pub enum ExtracterError {
    ChainError(ChainError, String),
    ParseError(Box<dyn Error>, String),
    Other(Box<dyn Error>, String),
}
impl Display for ExtracterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl Error for ExtracterError {}

pub trait Extracter {
    type Input;
    type Output;

    fn extract(
        &self,
        ctx: &Self::Input,
    ) -> impl std::future::Future<Output = Result<Self::Output, ExtracterError>> + Send;
}
