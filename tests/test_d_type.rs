use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;

// Example struct to deserialize into
#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    age: u32,
}

pub trait Extracter {
    // The async method that extracts and deserializes data into the type D.
    async fn extract<D>(&self, ctx: &HashMap<String, String>) -> Result<D, Box<dyn Error>>
    where
        D: DeserializeOwned; // Ensure that D is deserializable.
}

// An example implementation of the Extracter trait
struct MyExtractor;

impl Extracter for MyExtractor {
    // Extract method implementation
    async fn extract<D>(&self, ctx: &HashMap<String, String>) -> Result<D, Box<dyn Error>>
    where
        D: DeserializeOwned,
    {
        // Retrieve the JSON string from the HashMap (assuming the key is "data")
        if let Some(json_str) = ctx.get("data") {
            // Deserialize the string into the desired type D
            let deserialized_data: D = serde_json::from_str(json_str)?;
            Ok(deserialized_data)
        } else {
            Err("No data found in context".into())
        }
    }
}
