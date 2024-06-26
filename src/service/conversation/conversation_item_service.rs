use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rust_wheel::{
    common::{query::pagination::Paginate, util::model_convert::map_pagination_res},
    model::response::pagination_response::PaginationResponse,
};

use crate::{
    common::database::get_connection,
    model::{
        diesel::ai::custom_ai_models::ConversationItem,
        req::conversation::{
            conversation_item_add::ConversationItemAdd, conversation_item_req::ConversationItemReq,
        },
    },
};

pub fn conv_item_page(params: &ConversationItemReq) -> PaginationResponse<Vec<ConversationItem>> {
    use crate::model::diesel::ai::ai_schema::conversation_item as cv_tpl_table;
    let mut query = cv_tpl_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(cv_tpl_table::cid.eq(params.cid));
    let query = query
        .paginate(params.page_num.unwrap_or(1).clone())
        .per_page(params.page_size.unwrap_or(9).clone());
    let page_result: QueryResult<(Vec<ConversationItem>, i64, i64)> =
        query.load_and_count_pages_total::<ConversationItem>(&mut get_connection());
    let page_map_result = map_pagination_res(
        page_result,
        params.page_num.unwrap_or(1),
        params.page_size.unwrap_or(10),
    );
    return page_map_result;
}

pub fn create_conversation_item(prompt: &String, ans: &String, input_cid: i64) {
    use crate::model::diesel::ai::ai_schema::conversation_item::dsl::*;
    let new_conversation = ConversationItemAdd::gen_conversation_item(prompt, ans, input_cid);
    diesel::insert_into(conversation_item)
        .values(new_conversation)
        .get_result::<ConversationItem>(&mut get_connection())
        .expect("failed to add new conversation item");
}
