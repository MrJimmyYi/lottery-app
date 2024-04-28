use rusqlite::{self, params, Connection};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use std::sync::Arc;
use crate::db::{AppError, DbPool, UserCard, Result};


pub fn create_user(pool: &DbPool, userCard: &UserCard) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "INSERT INTO user_card (num, name, img, remark) VALUES (?1, ?2, ?3, ?4)",
        params![userCard.num, userCard.name, userCard.img, userCard.remark],
    )?;
    Ok(count)
}

pub fn create_batch_users(pool: &DbPool, data: Vec<UserCard>) -> Result<()> {
    let mut conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let tx = conn.transaction()?;

    for user_card in data {
        tx.execute(
            "INSERT INTO user_card (num, name, img, remark) VALUES (?1, ?2, ?3, ?4)",
            params![user_card.num, user_card.name, user_card.img, user_card.remark],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn select_user(pool: &DbPool, id: i32) -> Result<UserCard> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let user_card = conn.query_row(
        "SELECT id, num, name, img, remark FROM user_card WHERE id = ?1",
        params![id],
        |row| {
            Ok(UserCard {
                id: row.get(0)?,
                num: row.get(1)?,
                name: row.get(2)?,
                img: row.get(3)?,
                remark: row.get(4)?,
            })
        },
    )?;
    Ok(user_card)
}

pub fn delete_user(pool: &DbPool, id: i32) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute("DELETE FROM user_card WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn update_user(pool: &DbPool, userCard: &UserCard) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE user_card SET name = ?1, img = ?2, remark = ?3 WHERE id = ?4",
        params![userCard.name, userCard.img, userCard.remark, userCard.id],
    )?;
    Ok(count)
}

pub fn get_page_users(pool: &DbPool, page: usize, page_size: usize) -> Result<Vec<UserCard>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id, num, name, img, remark FROM user_card LIMIT ? OFFSET ?")?;
    let user_iter = stmt.query_map(params![page_size as i64, ((page - 1) * page_size) as i64], |row| {
        Ok(UserCard {
            id: row.get(0)?,
            num: row.get(1)?,
            name: row.get(2)?,
            img: row.get(3)?,
            remark: row.get(4)?
        })
    })?;
    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    Ok(users)
}

pub fn get_all_users(pool: &DbPool) -> Result<Vec<UserCard>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id, num, name, img, remark FROM user_card")?;
    let user_iter = stmt.query_map( params![], |row| {
        Ok(UserCard {
            id: row.get(0)?,
            num: row.get(1)?,
            name: row.get(2)?,
            img: row.get(3)?,
            remark: row.get(4)?
        })
    })?;
    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    Ok(users)
}

pub fn get_total_count_user(pool: &DbPool) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM user_card", [], |row| row.get(0))?;
    Ok(count as usize)
}