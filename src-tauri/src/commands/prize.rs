use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::api::path::app_data_dir;

use crate::{db, utils};
use crate::db::{AppError, DbPool, PagedData, Prize, TauriResponse, Result};

#[tauri::command]
pub fn create_prize(prize: Prize, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<String>> {
    let mut file = File::create(&prize.img)?;
    file.write_all(&img_data)?;
    db::prize::create_prize(&pool, &prize)?;
    Ok(TauriResponse{
        message: "Prize created successfully".into(),
        data: None
    })
}

#[tauri::command]
pub fn update_prize(prize: Prize, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(p) = db::prize::select_prize(&pool, prize.id) {
        //如果图片需要更新，则需要将原来的图片删除并替换成新的图片
        if !(prize.img == p.img) {
            fs::remove_file(p.img)?;
            let mut file = File::create(&prize.img)?;
            file.write_all(&img_data)?;
        }
        db::prize::update_prize(&pool, &prize)?;
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(prize.id)
    })
}

#[tauri::command]
pub fn update_prize_count(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    db::prize::update_prize_count(&pool,id)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(id)
    })
}


#[tauri::command]
pub fn delete_prize(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(p) = db::prize::select_prize(&pool, id) {
        db::prize::delete_prize(&pool, p.id)?;
        fs::remove_file(p.img)?;
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(id)
    })
}

#[tauri::command]
pub fn generate_imgstr(file_name: String, app_handle: tauri::AppHandle) -> Result<TauriResponse<String>> {
    let uploads_dir = app_data_dir(&app_handle.config())
        .ok_or(AppError::ApplicationError("无法获取应用目录".into()))? // 这里修正错误处理
        .join("img");
    let last_part = file_name.split('.').last().ok_or(AppError::ApplicationError("文件名无扩展名".into()))?;
    let name = format!("{}.{}", utils::generate_random_hash(16), last_part);
    Ok(TauriResponse{
        message: String::new(),
        data: Some(uploads_dir.join(name).to_string_lossy().into_owned())
    })
}


#[tauri::command]
pub fn get_page_prizes(pool: tauri::State<'_, DbPool>, page: usize, page_size: usize) -> Result<TauriResponse<PagedData<Prize>>> {
    let prizes = db::prize::get_page_prizes(&pool, page, page_size)?;
    let total = db::prize::get_total_count_prize(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(PagedData{ data: prizes, total })
    })
}

#[tauri::command]
pub fn get_all_prizes(pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Vec<Prize>>> {
    let prizes = db::prize::get_all_prizes(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(prizes)
    })
}

#[tauri::command]
pub fn get_prize(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Prize>> {
    Ok(TauriResponse{
        message: String::new(),
        data: Some(db::prize::select_prize(&pool, id)?)
    })
}