// 声明子模块
pub mod user;
pub mod prize;
pub mod winner;

pub mod basic;
use std::ptr::null;
use crate::db;
use crate::db::{create_card_table, create_model_basic_table, create_prize_table, create_winner_table};


pub fn init_app() -> Option<db::DbPool>{
    // 使用全局数据库连接进行操作
    let pool = match db::init_pool() {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize the database pool: {:?}", e);
            return None; // 或退出程序 std::process::exit(1);
        }
    };
    let _ = create_card_table(&pool);
    let _ = create_prize_table(&pool);
    let _ = create_model_basic_table(&pool);
    let _ = create_winner_table(&pool);
    return Some(pool);
}