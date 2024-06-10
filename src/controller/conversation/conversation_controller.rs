use crate::{model::req::conversation::conversation_req::ConversationReq, service::conversation::conversation_service::conv_page};
use actix_web::{get, web, HttpResponse};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// Ask
///
/// Ask
#[utoipa::path(
    context_path = "/ai/conversation/page",
    path = "/",
    responses(
        (status = 200, description = "conversation page")
    )
)]
#[get("/page")]
pub async fn conversation_page(
    params: actix_web_validator::Query<ConversationReq>,
) -> HttpResponse {
    let conv = conv_page(&params.0);
    return box_actix_rest_response(conv);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/conversation").service(conversation_page);
    conf.service(scope);
}
