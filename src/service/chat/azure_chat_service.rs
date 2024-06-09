use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_openai::{config::AzureConfig, types::ChatCompletionRequestSystemMessageArgs, Client};
use log::error;
use rust_wheel::common::util::net::sse_message::SSEMessage;
use std::env;
use std::error::Error;
use tokio::sync::mpsc::UnboundedSender;

pub async fn azure_chat(
    tx: &UnboundedSender<SSEMessage<String>>,
) -> Result<String, Box<dyn Error>> {
    let azure_chat_api_base = env::var("AZURE_CHAT_API_BASE").expect("AZURE_CHAT_API_BASE must be set");
    let deployment_id = env::var("DEPLOYMENT_ID").expect("DEPLOYMENT_ID must be set");
    let api_key = env::var("AZURE_OPENAI_KEY").expect("API_KEY must be set");
    let config = AzureConfig::new()
        .with_api_base(azure_chat_api_base)
        .with_api_version("2023-03-15-preview")
        .with_deployment_id(deployment_id)
        .with_api_key(api_key);
    let client = Client::with_config(config);
    return chat_completion_example(&client, tx).await;
}

async fn chat_completion_example(
    client: &Client<AzureConfig>,
    tx: &UnboundedSender<SSEMessage<String>>,
) -> Result<String, Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("How does large language model work?")
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
        let content = choice.message.content;
        if content.is_some() {
            do_msg_send_sync(&content.unwrap(), tx, "chat");
        }
    }
    Ok("".to_owned())
}

pub fn do_msg_send_sync(
    context: &String,
    tx: &UnboundedSender<SSEMessage<String>>,
    msg_type: &str,
) {
    let sse_msg: SSEMessage<String> =
        SSEMessage::from_data(context.to_string(), &msg_type.to_string());
    let send_result = tx.send(sse_msg);
    match send_result {
        Ok(_) => {}
        Err(e) => {
            error!("send chat response facing error: {}", e);
        }
    }
}
