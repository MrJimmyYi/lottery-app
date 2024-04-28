use rusqlite::params;
use crate::db::{AppError, DbPool, UserCard, Result, Winner};

pub fn add_winners(pool: &DbPool, winners: Vec<Winner>) -> Result<()> {
    let mut conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let tx = conn.transaction()?;
    for w in winners {
        tx.execute(
            "INSERT INTO winner (prize_range, prize_name, winner_num, winner_name) VALUES (?1, ?2, ?3, ?4)",
            params![w.prizeRange, w.prizeName, w.winnerNum, w.winnerName],
        )?;
    }
    tx.commit()?;
    Ok(())

}

pub fn get_all_winners(pool: &DbPool) -> Result<Vec<Winner>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id,prize_range,prize_name,winner_name,winner_num FROM winner")?;
    let winner_iter = stmt.query_map( params![], |row| {
        Ok(Winner {
            id: row.get(0)?,
            prizeRange: row.get(1)?,
            prizeName: row.get(2)?,
            winnerName: row.get(3)?,
            winnerNum: row.get(4)?
        })
    })?;
    let mut winners = Vec::new();
    for w in winner_iter {
        winners.push(w?);
    }
    Ok(winners)
}

pub fn get_page_winners(pool: &DbPool, page: usize, page_size: usize) -> Result<Vec<Winner>> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let mut stmt = conn.prepare("SELECT id, prize_name, prize_range, winner_name, winner_num FROM winner LIMIT ? OFFSET ?")?;
    let winner_iter = stmt.query_map(params![page_size as i64, ((page - 1) * page_size) as i64], |row| {
        Ok(Winner {
            id: row.get(0)?,
            prizeName: row.get(1)?,
            prizeRange: row.get(2)?,
            winnerName: row.get(3)?,
            winnerNum: row.get(4)?
        })
    })?;
    let mut winners = Vec::new();
    for w in winner_iter {
        winners.push(w?);
    }
    Ok(winners)
}

pub fn get_total_count_winners(pool: &DbPool) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM winner", [], |row| row.get(0))?;
    Ok(count as usize)
}