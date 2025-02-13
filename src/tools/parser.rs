use regex::Regex;
use serde_json::Value;
use std::error::Error as StdError;

pub fn parse_json_md(ctx: &str) -> Result<Value, Box<dyn StdError>> {
    let re = Regex::new(r"```json\s*(.*[\s\S]*?)\s*```").unwrap();
    let re_raw = Regex::new(r"\s*(\{.*[\s\S]*\})\s*").unwrap();

    if let Some(captures) = re.captures(ctx).or(re_raw.captures(ctx)) {
        let json_str = captures.get(1).unwrap().as_str();
        println!("Extracted JSON from code block: {}", json_str);
        let parsed_json: Value = serde_json::from_str(json_str)?;

        if let Value::Object(_) = parsed_json {
            Ok(parsed_json)
        } else {
            Err("JSON is not a dictionary".into())
        }
    } else {
        // No valid JSON found
        Err("No valid JSON block found".into())
    }
}

#[cfg(test)]
mod test {
    use super::parse_json_md;

    #[test]
    fn test_parse_json_md() {
        let md_content = r#"

        {
  "title": "Deep Residual Learning for Image Recognition",
  "solved_problem": ["Image recognition"],
  "research_field": ["Visual Recognition", "Computer Vision"],
  "techniques_used": ["Residual learning"]
    }
        "#
        .to_string();

        match parse_json_md(&md_content) {
            Ok(json) => println!("Parsed JSON: {}", json),
            Err(e) => println!("Error: {}", e),
        }
    }
}
