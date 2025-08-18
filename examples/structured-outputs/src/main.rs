use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let schema = json!({
      "type": "object",
      "properties": {
        "steps": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "explanation": { "type": "string" },
              "output": { "type": "string" }
            },
            "required": ["explanation", "output"],
            "additionalProperties": false
          }
        },
        "final_answer": { "type": "string" }
      },
      "required": ["steps", "final_answer"],
      "additionalProperties": false
    });

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: None,
            name: "math_reasoning".into(),
            schema: Some(schema),
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-5")
        .verbosity("medium")
        .reasoning_effort("minimal")
        .messages([
            ChatCompletionRequestSystemMessage::from(
                "You are a helpful assistant."
            ),
            ChatCompletionRequestUserMessage::from(
                "What is 2 + 2? Explain your reasoning step by step."
            ),
        ])
        .response_format(response_format)
        .tools(vec![serde_json::json!({"type": "custom", "name": "my_custom_tool", "description": "A custom tool for GPT-5"})])
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            print!("{content}")
        }
    }

    Ok(())
}
