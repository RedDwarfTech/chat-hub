use crate::{model::req::chat::ask_req::AskReq, service::chat::azure_chat_service::azure_chat};
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    web, HttpResponse,
};
use log::error;
use rust_wheel::common::util::net::{sse_message::SSEMessage, sse_stream::SseStream};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task,
};

/// Ask
/// https://stackoverflow.com/questions/77015804/why-the-event-source-polyfill-did-not-fetch-the-sse-api-data
/// Ask
#[utoipa::path(
    context_path = "/infra/user/login",
    path = "/",
    responses(
        (status = 200, description = "support user login")
    )
)]
#[get("/stream/chat/ask")]
pub async fn ask(_params: actix_web_validator::Query<AskReq>) -> HttpResponse {
    let (tx, rx): (
        UnboundedSender<SSEMessage<String>>,
        UnboundedReceiver<SSEMessage<String>>,
    ) = tokio::sync::mpsc::unbounded_channel();
    task::spawn(async move {
        let output = azure_chat(tx).await;
        if let Err(e) = output {
            error!("handle chat sse req error: {}", e);
        }
    });
    let response = HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/azure").service(ask);
    conf.service(scope);
}
