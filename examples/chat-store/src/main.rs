use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-5")
        .verbosity("medium")
        .reasoning_effort("minimal")
        .store(true)
        .metadata(json!({
            "role": "manager",
            "department": "accounting",
            "source": "homepage",
        }))
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a corporate IT support expert.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("How can I hide the dock on my Mac?")
                .build()?
                .into(),
        ])
        // Example custom tool usage for GPT-5
        .tools(vec![serde_json::json!({"type": "custom", "name": "my_custom_tool", "description": "A custom tool for GPT-5"})])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}
