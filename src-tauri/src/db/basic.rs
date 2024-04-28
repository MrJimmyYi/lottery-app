use rusqlite::params;
use crate::db::{AppError, BasicModel, DbPool, Prize, Result};

pub fn get_model_basic(pool: &DbPool, model: String) -> Result<BasicModel> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let basic = conn.query_row(
        "SELECT id,model,bg_img,audio_src,spin_time,re_drawing,remark1,remark2 FROM model_basic WHERE model = ?1",
        params![model],
        |row| {
            Ok(BasicModel {
                id: row.get(0)?,
                model: row.get(1)?,
                bgImg: row.get(2)?,
                audioSrc: row.get(3)?,
                spinTime: row.get(4)?,
                reDrawing: row.get(5)?,
                remark1: row.get(6)?,
                remark2: row.get(7)?
            })
        },
    )?;
    Ok(basic)
}

pub fn get_model_basic_by_id(pool: &DbPool, id: i32) -> Result<BasicModel> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let basic = conn.query_row(
        "SELECT id,model,bg_img,audio_src,spin_time,re_drawing,remark1,remark2 FROM model_basic WHERE id = ?1",
        params![id],
        |row| {
            Ok(BasicModel {
                id: row.get(0)?,
                model: row.get(1)?,
                bgImg: row.get(2)?,
                audioSrc: row.get(3)?,
                spinTime: row.get(4)?,
                reDrawing: row.get(5)?,
                remark1: row.get(6)?,
                remark2: row.get(7)?
            })
        },
    )?;
    Ok(basic)
}


pub fn update_basic_model(pool: &DbPool, basic: &BasicModel) -> Result<usize> {
    let conn = pool.get().map_err(|_| AppError::ConnectionPoolError)?;
    let count = conn.execute(
        "UPDATE model_basic SET bg_img=?1,audio_src=?2,spin_time=?3,re_drawing=?4 WHERE id = ?5",
        params![basic.bgImg, basic.audioSrc,   basic.spinTime, basic.reDrawing, basic.id],
    )?;
    Ok(count)
}