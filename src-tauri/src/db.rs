use rusqlite::{self, params, Connection};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use std::sync::Arc;
pub type DbPool = Arc<Pool<SqliteConnectionManager>>;
use directories::ProjectDirs;
use std::fs;
use crate::model::{AppError, Prize, UserCard};
pub type Result<T> = std::result::Result<T, AppError>;

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
        "CREATE TABLE IF NOT EXISTS userCard (
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
            perDraw INTEGER NOT NULL
         )",
        [],
    )?;
    Ok(())
}

// pub fn create_winner_table(pool: &DbPool) -> Result<()> {
//     let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS winner (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             range TEXT NOT NULL,
//             name TEXT NOT NULL,
//             img TEXT NOT NULL,
//             count INTEGER NOT NULL,
//             total INTEGER NOT NULL,
//             perDraw INTEGER NOT NULL
//          )",
//         [],
//     )?;
//     Ok(())
// }

pub fn create_user(pool: &DbPool, userCard: &UserCard) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "INSERT INTO userCard (num, name, img, remark) VALUES (?1, ?2, ?3, ?4)",
        params![userCard.num, userCard.name, userCard.img, userCard.remark],
    )?;
    Ok(count)
}

pub fn create_batch_users(pool: &DbPool, data: Vec<UserCard>) -> Result<()> {
    let mut conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let tx = conn.transaction()?;

    for user_card in data {
        tx.execute(
            "INSERT INTO userCard (num, name, img, remark) VALUES (?1, ?2, ?3, ?4)",
            params![user_card.num, user_card.name, user_card.img, user_card.remark],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn select_user(pool: &DbPool, id: i32) -> Result<UserCard> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let user_card = conn.query_row(
        "SELECT id, num, name, img, remark FROM userCard WHERE id = ?1",
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
    conn.execute("DELETE FROM userCard WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn update_user(pool: &DbPool, userCard: &UserCard) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE userCard SET name = ?1, img = ?2, remark = ?3 WHERE id = ?4",
        params![userCard.name, userCard.img, userCard.remark, userCard.id],
    )?;
    Ok(count)
}

pub fn get_page_users(pool: &DbPool, page: usize, page_size: usize) -> Result<Vec<UserCard>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id, num, name, img, remark FROM userCard LIMIT ? OFFSET ?")?;
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
    let mut stmt = conn.prepare("SELECT id, num, name, img, remark FROM userCard")?;
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
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM userCard", [], |row| row.get(0))?;
    Ok(count as usize)
}

pub fn update_prize(pool: &DbPool, prize: &Prize) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE prize SET range=?1,name=?2,img=?3,count=?4,total=?5,perDraw=?6 WHERE id = ?7",
        params![prize.range, prize.name, prize.img,  prize.count, prize.total, prize.perDraw, prize.id],
    )?;
    Ok(count)
}

pub fn create_prize(pool: &DbPool, prize: &Prize) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "INSERT INTO prize (range,name,img,count,total,perDraw) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![prize.range, prize.name, prize.img,  prize.count, prize.total, prize.perDraw],
    )?;
    Ok(count)
}

pub fn delete_prize(pool: &DbPool, id: i32) -> Result<()> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    conn.execute("DELETE FROM prize WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_page_prizes(pool: &DbPool, page: usize, page_size: usize) -> Result<Vec<Prize>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id,range,name,img,count,total,perDraw FROM prize LIMIT ? OFFSET ?")?;
    let prize_iter = stmt.query_map(params![page_size as i64, ((page - 1) * page_size) as i64], |row| {
        Ok(Prize {
            id: row.get(0)?,
            range: row.get(1)?,
            name: row.get(2)?,
            img: row.get(3)?,
            count: row.get(4)?,
            total: row.get(5)?,
            perDraw: row.get(6)?
        })
    })?;
    let mut prizes = Vec::new();
    for prize in prize_iter {
        prizes.push(prize?);
    }
    Ok(prizes)
}

pub fn get_all_prizes(pool: &DbPool) -> Result<Vec<Prize>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id,range,name,img,count,total,perDraw FROM prize")?;
    let prize_iter = stmt.query_map( params![], |row| {
        Ok(Prize {
            id: row.get(0)?,
            range: row.get(1)?,
            name: row.get(2)?,
            img: row.get(3)?,
            count: row.get(4)?,
            total: row.get(5)?,
            perDraw: row.get(6)?
        })
    })?;
    let mut prizes = Vec::new();
    for prize in prize_iter {
        prizes.push(prize?);
    }
    Ok(prizes)
}

pub fn select_prize(pool: &DbPool, id: i32) -> Result<Prize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let prize = conn.query_row(
        "SELECT id,range,name,img,count,total,perDraw FROM prize WHERE id = ?1",
        params![id],
        |row| {
            Ok(Prize {
                id: row.get(0)?,
                range: row.get(1)?,
                name: row.get(2)?,
                img: row.get(3)?,
                count: row.get(4)?,
                total: row.get(5)?,
                perDraw: row.get(6)?
            })
        },
    )?;
    Ok(prize)
}

pub fn get_total_count_prize(pool: &DbPool) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM prize", [], |row| row.get(0))?;
    Ok(count as usize)
}

pub fn update_prize_count(pool: &DbPool, id: i32) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE prize SET count=count+perDraw WHERE id = ?1",
        params![id],
    )?;
    Ok(count)
}

