use rust_wheel::model::error::error_response::ErrorResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("超出每天次数限制")]
    ExceedTheChatPerDayLimit,
}
impl ErrorResponse for ChatError {
    fn error_code(&self) -> &str {
        match self {
            ChatError::ExceedTheChatPerDayLimit => "0040010001",
        }
    }

    fn error_message(&self) -> &str {
        match self {
            ChatError::ExceedTheChatPerDayLimit => "超出每天次数限制",
        }
    }

    fn error_code_en(&self) -> &str {
        match self {
            ChatError::ExceedTheChatPerDayLimit => "EXCEED_THE_CHAT_PER_DAY_LIMIT",
        }
    }
}
