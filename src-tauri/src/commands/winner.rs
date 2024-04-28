use crate::db;
use crate::db::{DbPool, PagedData, TauriResponse, Winner, Result};

#[tauri::command]
pub fn create_winners(winners: Vec<Winner>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<String>> {
    db::winner::add_winners(&pool, winners)?;
    Ok(TauriResponse{
        message: "winners created successfully".into(),
        data: None
    })
}

#[tauri::command]
pub fn get_all_winners(pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Vec<Winner>>> {
    let winners = db::winner::get_all_winners(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(winners)
    })
}

#[tauri::command]
pub fn get_page_winners(pool: tauri::State<'_, DbPool>, page: usize, page_size: usize) -> Result<TauriResponse<PagedData<Winner>>> {
    let winners = db::winner::get_page_winners(&pool, page, page_size)?;
    let total = db::winner::get_total_count_winners(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(PagedData{ data: winners, total })
    })
}