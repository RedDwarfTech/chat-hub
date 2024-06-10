use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct AskReq {
    pub prompt: String,
    pub cid: Option<i64>,
}