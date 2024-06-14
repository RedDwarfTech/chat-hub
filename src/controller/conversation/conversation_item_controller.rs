use crate::{
    model::req::conversation::conversation_item_req::ConversationItemReq,
    service::conversation::conversation_item_service::conv_item_page,
};
use actix_web::{get, web, HttpResponse};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// Conversation item
///
/// Conversation item
#[utoipa::path(
    context_path = "/ai/conversation/item/page",
    path = "/",
    responses(
        (status = 200, description = "conversation item page")
    )
)]
#[get("/page")]
pub async fn conversation_item_page(
    params: actix_web_validator::Query<ConversationItemReq>,
) -> HttpResponse {
    let conv = conv_item_page(&params.0);
    return box_actix_rest_response(conv);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/ai/conversation/item").service(conversation_item_page);
    conf.service(scope);
}
