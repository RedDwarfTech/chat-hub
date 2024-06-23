use crate::{
    model::req::chat::ask_req::AskReq,
    service::chat::azure_chat_service::{azure_chat, do_msg_send_sync},
};
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    web, HttpResponse,
};
use log::error;
use rust_wheel::{
    common::util::{
        net::{sse_message::SSEMessage, sse_stream::SseStream},
        time_util::get_current_millisecond,
    },
    config::{
        app::app_conf_reader::get_app_config,
        cache::redis_util::{incre_redis_key, set_str, sync_get_str},
    },
    model::user::login_user_info::LoginUserInfo,
};
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
pub async fn ask(
    params: actix_web_validator::Query<AskReq>,
    login_user_info: LoginUserInfo,
) -> HttpResponse {
    if login_user_info.vipExpireTime >= get_current_millisecond() {
        return handle_chat(&params.0, &login_user_info);
    }
    let login_failed_key = get_app_config("chat.chat_per_day_key");
    let user_failed_key = format!("{}:{}", login_failed_key, login_user_info.userId);
    let chat_count: Option<String> = sync_get_str(&user_failed_key);
    if chat_count.is_none() {
        increase_chaht_count(&user_failed_key);
        return handle_chat(&params.0, &login_user_info);
    }
    if chat_count.unwrap().parse::<i32>().unwrap() > 20 {
        let (tx, rx): (
            UnboundedSender<SSEMessage<String>>,
            UnboundedReceiver<SSEMessage<String>>,
        ) = tokio::sync::mpsc::unbounded_channel();
        do_msg_send_sync(&"vip-expire".to_string(), &tx, "chat");
        let response = HttpResponse::Ok()
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
            .content_type("text/event-stream")
            .streaming(SseStream { receiver: Some(rx) });
        response
    } else {
        increase_chaht_count(&user_failed_key);
        return handle_chat(&params.0, &login_user_info);
    }
}

fn handle_chat(req: &AskReq, login_user_info: &LoginUserInfo) -> HttpResponse {
    let (tx, rx): (
        UnboundedSender<SSEMessage<String>>,
        UnboundedReceiver<SSEMessage<String>>,
    ) = tokio::sync::mpsc::unbounded_channel();
    let req1 = req.clone();
    let lu = login_user_info.clone();
    task::spawn(async move {
        let output = azure_chat(tx, &req1, &lu).await;
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

fn increase_chaht_count(cached_key: &String) {
    let app_str = sync_get_str(&cached_key);
    if app_str.is_none() {
        set_str(&cached_key, "1", 86400);
    } else {
        let incre_result = incre_redis_key(&cached_key, 1);
        if let Err(err) = incre_result {
            error!("increment login count failed, {}", err)
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/azure").service(ask);
    conf.service(scope);
}
