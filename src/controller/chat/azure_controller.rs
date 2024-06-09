use crate::model::req::chat::ask_req::AskReq;
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
///
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
        do_msg_send_sync(&"test".to_string(), &tx, "chat");
    });
    let response = HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .content_type("text/event-stream")
        .streaming(SseStream { receiver: Some(rx) });
    response
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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/azure").service(ask);
    conf.service(scope);
}
