use rust_wheel::common::util::time_util::get_current_millisecond;
use serde::{Deserialize, Serialize};
use crate::model::diesel::ai::ai_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = conversation)]
pub struct ConversationAdd {
    pub created_time: i64,
    pub title: String,
    pub updated_time: i64,
    pub user_id: i64,
}

impl ConversationAdd {
    pub(crate) fn gen_conversation(prompt: &String, uid: &i64) ->Self {
        Self {
            created_time: get_current_millisecond(),
            title: prompt.to_string(),
            updated_time: get_current_millisecond(),
            user_id: uid.to_owned()
        }
    }
}