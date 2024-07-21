table! {
    conversation (id) {
        id -> Int8,
        created_time -> Int8,
        title -> Varchar,
        updated_time -> Int8,
        user_id -> Int8,
    }
}

table! {
    conversation_item (id) {
        id -> Int8,
        created_time -> Int8,
        question -> Nullable<Text>,
        updated_time -> Nullable<Int8>,
        answer -> Nullable<Text>,
        question_time -> Nullable<Int8>,
        answer_time -> Nullable<Int8>,
        cid -> Int8,
        req_id -> Nullable<Varchar>,
        user_id -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    conversation,
    conversation_item,
);
