use crate::model::req::chat::ask_req::AskReq;
use crate::service::conversation::conversation_item_service::create_conversation_item;
use crate::service::conversation::conversation_service::create_conversation;
use async_openai::types::{
    ChatCompletionRequestUserMessageArgs, Choice, CompletionFinishReason,
    CreateChatCompletionRequestArgs, CreateChatCompletionResponse, CreateCompletionRequestArgs,
    CreateCompletionResponse,
};
use async_openai::{config::AzureConfig, types::ChatCompletionRequestSystemMessageArgs, Client};
use futures::StreamExt;
use log::error;
use rust_wheel::common::util::net::sse_message::SSEMessage;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use std::env;
use std::error::Error;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub async fn azure_chat(
    tx: UnboundedSender<SSEMessage<String>>,
    req: &AskReq,
    login_user_info: &LoginUserInfo,
) -> Result<String, Box<dyn Error>> {
    let azure_chat_api_base =
        env::var("AZURE_CHAT_API_BASE").expect("AZURE_CHAT_API_BASE must be set");
    let deployment_id = env::var("DEPLOYMENT_ID").expect("DEPLOYMENT_ID must be set");
    let api_key = env::var("AZURE_OPENAI_KEY").expect("AZURE_OPENAI_KEY must be set");
    let config = AzureConfig::new()
        .with_api_base(azure_chat_api_base)
        .with_api_version("2023-03-15-preview")
        .with_deployment_id(deployment_id)
        .with_api_key(api_key);
    let client = Client::with_config(config);
    return chat_completion(&client, tx, req, login_user_info).await;
}

async fn chat_completion(
    client: &Client<AzureConfig>,
    tx: UnboundedSender<SSEMessage<String>>,
    req: &AskReq,
    login_user_info: &LoginUserInfo,
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
                .content(req.prompt.as_str())
                .build()?
                .into(),
        ])
        .build()?;

    let response: CreateChatCompletionResponse = client.chat().create(request).await?;
    let msg_string = serde_json::to_string(&response);
    if let Err(e) = msg_string {
        error!("serial json failed,{}", e);
        return Ok("".to_owned());
    }
    let send_msg = msg_string.unwrap();
    do_msg_send_sync(&send_msg, &tx, "chat");
    let choice = response.choices[0].clone();
    let msg = choice.message.content.unwrap_or_default();
    if req.cid.is_none() || req.cid.unwrap() == 0 {
        let conv = create_conversation(&req.prompt, &login_user_info.userId);
        create_conversation_item(&req.prompt, &msg, conv.id, login_user_info.userId);
    }
    create_conversation_item(&req.prompt, &msg, req.cid.unwrap(), login_user_info.userId);
    Ok("".to_owned())
}

/**
 * https://github.com/64bit/async-openai/pull/67#issuecomment-1555165805
 */
async fn _chat_completion_stream(
    client: &Client<AzureConfig>,
    tx: UnboundedSender<SSEMessage<String>>,
    req: &AskReq,
) -> Result<String, Box<dyn Error>> {
    let request = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .n(1)
        .prompt(req.prompt.as_str())
        .stream(true)
        .max_tokens(512u16)
        .build()?;

    let mut stream = client.completions().create_stream(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => {
                let msg_string = serde_json::to_string(&ccr);
                if let Err(e) = msg_string {
                    error!("serial json failed,{}", e);
                    return Ok("".to_owned());
                }
                do_msg_send_sync(&msg_string.unwrap(), &tx, "chat");
            }
            Err(e) => error!("create stream facing issue, {}", e),
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

pub fn do_custom_msg_send_sync(
    context: &String,
    tx: &UnboundedSender<SSEMessage<String>>,
    msg_type: &str,
) {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    let mut ch: Vec<Choice> = Vec::new();
    let choice = Choice {
        text: context.to_string(),
        index: 0,
        logprobs: None,
        finish_reason: Some(CompletionFinishReason::Stop),
    };
    ch.push(choice);
    let msg = CreateCompletionResponse {
        id: uuid_string,
        choices: ch,
        created: get_current_millisecond() as u32,
        model: "3.5-turbo".to_string(),
        system_fingerprint: None,
        object: "".to_string(),
        usage: None,
    };
    let msg_string = serde_json::to_string(&msg);
    let sse_msg: SSEMessage<String> =
        SSEMessage::from_data(msg_string.unwrap().to_string(), &msg_type.to_string());
    let send_result = tx.send(sse_msg);
    match send_result {
        Ok(_) => {}
        Err(e) => {
            error!("send chat response facing error: {}", e);
        }
    }
}
