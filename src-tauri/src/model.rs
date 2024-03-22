use serde::{Deserialize, Serialize};
use thiserror::Error;
use rusqlite::{self, params, Connection};
use tauri::InvokeError;

#[derive(Serialize, Deserialize)]
pub struct TauriResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(default)] // 为所有字段应用默认值
pub struct UserCard {
    pub id: i32,
    pub num: String,
    pub name: String,
    pub img: String,
    pub remark: String,
}


#[derive(Serialize, Deserialize,Debug)]
#[serde(default)] // 为所有字段应用默认值
pub struct Prize {
    pub id: i32,
    pub range: String,
    pub name: String,
    pub img: String,
    pub count: i32,
    pub total: i32,
    pub perDraw: i32
}

// 防止json字符串转成UserCard的时候id为0默认值
impl Default for UserCard {
    fn default() -> UserCard {
        UserCard {
            id: 0, // 提供默认值
            num: String::new(),
            name: String::new(),
            img: String::new(),
            remark: String::new(),
        }
    }
}

impl Default for Prize {
    fn default() -> Prize {
        Prize {
            id: 0, // 提供默认值
            range: String::new(),
            name: String::new(),
            img: String::new(),
            count: 0,
            total: 0,
            perDraw: 0
        }
    }
}

#[derive(Serialize)]
pub struct PagedData<T> {
    pub data: Vec<T>,
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


