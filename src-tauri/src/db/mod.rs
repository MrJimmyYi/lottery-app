
use std::fs;
use std::sync::Arc;
use directories::ProjectDirs;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{InvokeError, State};
use thiserror::Error;

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


#[derive(Serialize, Deserialize,Debug)]
#[serde(default)] // 为所有字段应用默认值
pub struct Winner {
    pub id: i32,
    pub prizeRange: String,
    pub prizeName: String,
    pub winnerNum: String,
    pub winnerName: String
}

impl Default for Winner {
    fn default() -> Winner {
        Winner {
            id: 0, // 提供默认值
            prizeRange: String::new(),
            prizeName: String::new(),
            winnerNum: String::new(),
            winnerName: String::new()
        }
    }
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

#[derive(Serialize, Deserialize,Debug)]
#[serde(default)] // 为所有字段应用默认值
pub struct BasicModel {
    pub id: i32,
    pub model: String,
    pub bgImg: String,
    pub audioSrc: String,
    pub spinTime: i32,
    pub reDrawing: i32,
    pub remark1: String,
    pub remark2: String,
}

impl Default for BasicModel {
    fn default() -> BasicModel {
        BasicModel {
            id: 0, // 提供默认值
            model: String::new(),
            bgImg: String::new(),
            audioSrc: String::new(),
            remark1: String::new(),
            remark2: String::new(),
            spinTime: 10,
            reDrawing: 1,
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

pub type Result<T> = std::result::Result<T, AppError>;
pub type DbPool = Arc<Pool<SqliteConnectionManager>>;

pub fn init_pool() -> Result<DbPool> {
    let proj_dirs = ProjectDirs::from("com", "tauri", "build")
        .ok_or(AppError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to find project dirs")))?;
    let app_data_dir = proj_dirs.data_dir();

    fs::create_dir_all(app_data_dir).map_err(AppError::IOError)?;

    let database_dir = app_data_dir.join("database");
    fs::create_dir_all(&database_dir).map_err(AppError::IOError)?;

    let db_file_path = database_dir.join("lottery.db");
    println!("Database file path: {}", db_file_path.display());

    let manager = SqliteConnectionManager::file(db_file_path);
    let pool = Pool::new(manager).map_err(|_| AppError::ConnectionPoolError)?;

    Ok(Arc::new(pool))
}

pub fn create_card_table(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_card (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            num TEXT NOT NULL,
            name TEXT NOT NULL,
            img TEXT NOT NULL,
            remark TEXT
         )",
        [],
    )?;
    Ok(())
}

pub fn create_model_basic_table(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS model_basic (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            model TEXT NOT NULL UNIQUE,
            bg_img TEXT DEFAULT '',
            audio_src TEXT DEFAULT '',
            audio_title TEXT DEFAULT '',
            spin_time  INTEGER NOT NULL DEFAULT 10,
            re_drawing INTEGER NOT NULL DEFAULT 1,
            remark1 TEXT DEFAULT '',
            remark2 TEXT DEFAULT ''
         )",
        [],
    )?;
    // defaut insert data for wedding model
    conn.execute(
        "INSERT OR IGNORE INTO model_basic (model, audio_title) VALUES (?1, ?2)",
        params!["wedding", "Music"],
    )?;
    Ok(())
}

pub fn create_prize_table(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prize (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            range TEXT NOT NULL,
            name TEXT NOT NULL,
            img TEXT NOT NULL,
            count INTEGER NOT NULL,
            total INTEGER NOT NULL,
            per_draw INTEGER NOT NULL
         )",
        [],
    )?;
    Ok(())
}

pub fn create_winner_table(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS winner (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prize_range TEXT NOT NULL,
            prize_name TEXT NOT NULL,
            winner_num TEXT NOT NULL,
            winner_name TEXT NOT NULL
         )",
        [],
    )?;
    Ok(())
}


// 声明子模块
pub mod user;
pub mod prize;
pub mod winner;

pub mod basic;
