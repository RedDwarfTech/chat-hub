use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct ConversationReq {
    pub title: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}