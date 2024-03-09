use serde::{Deserialize, Serialize};
use thiserror::Error;
use rusqlite::{self, params, Connection};
use tauri::InvokeError;

#[derive(Serialize, Deserialize,Debug)]
#[serde(default)] // 为所有字段应用默认值
pub struct UserCard {
    pub id: i32,
    pub num: String,
    pub name: String,
    pub img: String,
    pub remark1: String,
    pub remark2: String,
}

// 防止json字符串转成UserCard的时候id为0默认值
impl Default for UserCard {
    fn default() -> UserCard {
        UserCard {
            id: 0, // 提供默认值
            num: String::new(),
            name: String::new(),
            img: String::new(),
            remark1: String::new(),
            remark2: String::new(),
        }
    }
}

#[derive(Serialize)]
pub struct PagedUserResponse {
    pub users: Vec<UserCard>,
    pub total: usize,
}


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Connection Pool Error")]
    ConnectionPoolError,
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to execute external command: {0}")]
    CommandError(String),
    #[error("Application error: {0}")]
    ApplicationError(String),
    #[error("Serde JSON Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<AppError> for InvokeError {
    fn from(error: AppError) -> Self {
        // 这里使用 error.to_string() 来获取错误描述作为字符串
        // 您可以根据需要调整错误消息的格式
        InvokeError::from(error.to_string())
    }
}


