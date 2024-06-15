use rust_wheel::common::util::time_util::get_current_millisecond;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::diesel::ai::ai_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = conversation_item)]
pub struct ConversationItemAdd {
    pub created_time: i64,
    pub question: Option<String>,
    pub updated_time: Option<i64>,
    pub answer: Option<String>,
    pub question_time: Option<i64>,
    pub answer_time: Option<i64>,
    pub cid: i64,
    pub req_id: Option<String>,
}

impl ConversationItemAdd {
    pub(crate) fn gen_conversation_item(prompt: &String, answer: &String, cid: i64) ->Self {
        let uuid = Uuid::new_v4();
        let uuid_string = uuid.to_string().replace("-", "");
        Self {
            created_time: get_current_millisecond(),
            updated_time: Some(get_current_millisecond()),
            question: Some(prompt.to_string()),
            answer: Some(answer.to_string()),
            question_time: Some(get_current_millisecond()),
            answer_time: Some(get_current_millisecond()),
            cid: cid,
            req_id: Some(uuid_string),
        }
    }
}