use validator::Validate;

#[derive(serde::Deserialize, Validate, Clone)]
#[allow(non_snake_case)]
pub struct AskReq {
    pub prompt: String,
    pub cid: Option<i64>,
}