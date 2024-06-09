use diesel::{QueryDsl, QueryResult, TextExpressionMethods};
use rust_wheel::{common::{query::pagination::Paginate, util::model_convert::map_pagination_res}, model::response::pagination_response::PaginationResponse};

use crate::{common::database::get_connection, model::{
    diesel::ai::custom_ai_models::Conversation,
    req::conversation::conversation_req::ConversationReq,
}};

pub fn conv_page(params: &ConversationReq) -> PaginationResponse<Vec<Conversation>> {
    use crate::model::diesel::ai::ai_schema::conversation as cv_tpl_table;
    let mut query = cv_tpl_table::table.into_boxed::<diesel::pg::Pg>();
    if params.title.as_ref().is_some() {
        query =
            query.filter(cv_tpl_table::title.like(format!("%{}%", params.title.as_ref().unwrap())));
    }
    let query = query
        .paginate(params.page_num.unwrap_or(1).clone())
        .per_page(params.page_size.unwrap_or(9).clone());
    let page_result: QueryResult<(Vec<Conversation>, i64, i64)> =
        query.load_and_count_pages_total::<Conversation>(&mut get_connection());
    let page_map_result = map_pagination_res(
        page_result,
        params.page_num.unwrap_or(1),
        params.page_size.unwrap_or(10),
    );
    return page_map_result;
}
