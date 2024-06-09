use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_openai::{config::AzureConfig, types::ChatCompletionRequestSystemMessageArgs, Client};
use log::error;
use rust_wheel::common::util::net::sse_message::SSEMessage;
use std::env;
use std::error::Error;
use tokio::sync::mpsc::UnboundedSender;

use crate::model::req::chat::ask_req::AskReq;

pub async fn azure_chat(tx: UnboundedSender<SSEMessage<String>>, req: &AskReq) -> Result<String, Box<dyn Error>> {
    let azure_chat_api_base =
        env::var("AZURE_CHAT_API_BASE").expect("AZURE_CHAT_API_BASE must be set");
    let deployment_id = env::var("DEPLOYMENT_ID").expect("DEPLOYMENT_ID must be set");
    let api_key = env::var("AZURE_OPENAI_KEY").expect("API_KEY must be set");
    let config = AzureConfig::new()
        .with_api_base(azure_chat_api_base)
        .with_api_version("2023-03-15-preview")
        .with_deployment_id(deployment_id)
        .with_api_key(api_key);
    let client = Client::with_config(config);
    return chat_completion(&client, tx, req).await;
}

async fn chat_completion(
    client: &Client<AzureConfig>,
    tx: UnboundedSender<SSEMessage<String>>,
    req: &AskReq
) -> Result<String, Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .stream(true)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(req.prompt.as_str())
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let msg_string = serde_json::to_string(&response);
    if let Err(e) = msg_string {
        error!("serial json failed,{}", e);
        return Ok("".to_owned());
    }
    do_msg_send_sync(&msg_string.unwrap(), &tx, "chat");
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
