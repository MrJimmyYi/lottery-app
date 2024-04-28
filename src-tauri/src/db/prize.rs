use rusqlite::params;

use crate::db::{AppError, DbPool, Prize, Result};

pub fn update_prize(pool: &DbPool, prize: &Prize) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE prize SET range=?1,name=?2,img=?3,count=?4,total=?5,per_draw=?6 WHERE id = ?7",
        params![prize.range, prize.name, prize.img,  prize.count, prize.total, prize.perDraw, prize.id],
    )?;
    Ok(count)
}

pub fn create_prize(pool: &DbPool, prize: &Prize) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "INSERT INTO prize (range,name,img,count,total,per_draw) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
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
    let mut stmt = conn.prepare("SELECT id,range,name,img,count,total,per_draw FROM prize LIMIT ? OFFSET ?")?;
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
    let mut stmt = conn.prepare("SELECT id,range,name,img,count,total,per_draw FROM prize")?;
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
        "SELECT id,range,name,img,count,total,per_draw FROM prize WHERE id = ?1",
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
        "UPDATE prize SET count=count+per_draw WHERE id = ?1",
        params![id],
    )?;
    Ok(count)
}