use crate::{
    common::error::chat_error::ChatError,
    model::req::chat::ask_req::AskReq,
    service::{
        chat::azure_chat_service::{azure_chat, do_custom_msg_send_sync},
        conversation::conversation_item_service::count_today_chat,
    },
};
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    web, HttpResponse,
};
use log::error;
use rust_wheel::{
    common::{
        util::{
            net::{sse_message::SSEMessage, sse_stream::SseStream},
            time_util::get_current_millisecond,
        },
        wrapper::actix_http_resp::box_err_actix_rest_response,
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
    let chat_perday_key = get_app_config("chat.chat_per_day_key");
    let user_failed_key = format!("{}:{}", chat_perday_key, login_user_info.userId);
    let chat_count: Option<String> = sync_get_str(&user_failed_key);
    if chat_count.is_none() {
        increase_user_chat_count(&user_failed_key);
        return handle_chat(&params.0, &login_user_info);
    }
    if chat_count.unwrap().parse::<i32>().unwrap() > 2 {
        let (tx, rx): (
            UnboundedSender<SSEMessage<String>>,
            UnboundedReceiver<SSEMessage<String>>,
        ) = tokio::sync::mpsc::unbounded_channel();
        do_custom_msg_send_sync(&"vip-expired".to_string(), &tx, "chat");
        let response = HttpResponse::Ok()
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
            .content_type("text/event-stream")
            .streaming(SseStream { receiver: Some(rx) });
        response
    } else {
        increase_user_chat_count(&user_failed_key);
        return handle_chat(&params.0, &login_user_info);
    }
}

fn handle_chat(req: &AskReq, login_user_info: &LoginUserInfo) -> HttpResponse {
    let (tx, rx): (
        UnboundedSender<SSEMessage<String>>,
        UnboundedReceiver<SSEMessage<String>>,
    ) = tokio::sync::mpsc::unbounded_channel();
    let ask_req = req.clone();
    let lu = login_user_info.clone();
    let today_chat_count = count_today_chat(&lu.userId);
    if today_chat_count > 200 {
        return box_err_actix_rest_response(ChatError::ExceedTheChatPerDayLimit);
    }
    task::spawn(async move {
        let output = azure_chat(tx, &ask_req, &lu).await;
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

fn increase_user_chat_count(cached_key: &String) {
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
