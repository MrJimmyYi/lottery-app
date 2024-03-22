use std::env;
use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use crate::db;
use crate::db::DbPool;
use crate::model::{AppError, PagedData, Prize, TauriResponse, UserCard};
use crate::utils::generate_random_hash;
use thiserror::Error;
use std::fs;
use tauri::CursorIcon::Default;

pub type Result<T> = std::result::Result<T, AppError>;

#[tauri::command]
pub fn create_prize(prize: Prize, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<String>> {
    let mut file = File::create(&prize.img)?;
    file.write_all(&img_data)?;
    db::create_prize(&pool, &prize)?;
    Ok(TauriResponse{
        message: "Prize created successfully".into(),
        data: None
    })
}

#[tauri::command]
pub fn create_user(user: UserCard, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<String>> {
    let mut file = File::create(&user.img)?;
    file.write_all(&img_data)?;
    db::create_user(&pool, &user)?;
    Ok(TauriResponse{
        message: "User created successfully".into(),
        data: None
    })
}

#[tauri::command]
pub fn update_user(user: UserCard, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(userCard) = db::select_user(&pool, user.id) {
        db::update_user(&pool, &user)?;
        //如果图片需要更新，则需要将原来的图片删除并替换成新的图片
        if !(user.img == userCard.img) {
            fs::remove_file(userCard.img)?;
            let mut file = File::create(&user.img)?;
            file.write_all(&img_data)?;
        }
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(user.id)
    })
}

#[tauri::command]
pub fn update_prize(prize: Prize, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(p) = db::select_prize(&pool, prize.id) {
        db::update_prize(&pool, &prize)?;
        //如果图片需要更新，则需要将原来的图片删除并替换成新的图片
        if !(prize.img == p.img) {
            fs::remove_file(p.img)?;
            let mut file = File::create(&prize.img)?;
            file.write_all(&img_data)?;
        }
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(prize.id)
    })
}

#[tauri::command]
pub fn update_prize_count(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    db::update_prize_count(&pool,id)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(id)
    })
}

#[tauri::command]
pub fn delete_user(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(userCard) = db::select_user(&pool, id) {
        db::delete_user(&pool, userCard.id)?;
        fs::remove_file(userCard.img)?;
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(id)
    })
}

#[tauri::command]
pub fn delete_prize(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(p) = db::select_prize(&pool, id) {
        db::delete_prize(&pool, p.id)?;
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
    let name = format!("{}.{}", generate_random_hash(16), last_part);
    Ok(TauriResponse{
        message: String::new(),
        data: Some(uploads_dir.join(name).to_string_lossy().into_owned())
    })
}


#[tauri::command]
pub fn batch_excel_operate(pool: tauri::State<'_, DbPool>, file_name: String, file_content: Vec<u8>, app_handle: tauri::AppHandle) -> Result<TauriResponse<String>> {
    let tmp_dir = temp_dir().join("file");
    let name = format!("{}.{}", generate_random_hash(16), file_name.split('.').last().ok_or(AppError::ApplicationError("文件名无扩展名".into()))?);
    let temp_file_path = tmp_dir.join(&name);
    fs::write(&temp_file_path, &file_content)?;

    let uploads_dir = app_data_dir(&app_handle.config())
        .ok_or(AppError::ApplicationError("无法获取应用目录".into()))? // 这里修正错误处理
        .join("img");

    let binary_path = match env::consts::OS {
        "linux" => "op_excel_linux",
        "macos" => "op_excel_mac",
        "windows" => "op_excel_windows.exe",
        _ => return Err(AppError::ApplicationError("Unsupported OS".into())),
    };

    let binary = std::env::current_exe()?.parent().ok_or(AppError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "Executable parent not found")))?.join("bin").join(binary_path);

    let output = std::process::Command::new(binary)
        .arg(&temp_file_path)
        .arg(&uploads_dir)
        .output()
        .map_err(|e| AppError::CommandError(e.to_string()))?;

    if !output.status.success() {
        return Err(AppError::CommandError("Command execution failed".into()));
    }

    let data = String::from_utf8_lossy(&output.stdout).to_string();
    let users: Vec<UserCard> = serde_json::from_str(&data)?;
    db::create_batch_users(&pool, users)?;

    Ok(TauriResponse{
        message: String::new(),
        data: Some(temp_file_path.to_string_lossy().into_owned())
    })

}

#[tauri::command]
pub fn download_template_excel(file_name: String) ->  Result<TauriResponse<Vec<u8>>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(format!("{}.xlsx", file_name));
    let buffer = std::fs::read(&path)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(buffer)
    })
}

#[tauri::command]
pub fn get_page_users(pool: tauri::State<'_, DbPool>, page: usize, page_size: usize) -> Result<TauriResponse<PagedData<UserCard>>> {
    let users = db::get_page_users(&pool, page, page_size)?;
    let total = db::get_total_count_user(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(PagedData{ data: users, total })
    })
}

#[tauri::command]
pub fn get_all_users(pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Vec<UserCard>>> {
    let users = db::get_all_users(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(users)
    })
}

#[tauri::command]
pub fn get_page_prizes(pool: tauri::State<'_, DbPool>, page: usize, page_size: usize) -> Result<TauriResponse<PagedData<Prize>>> {
    let prizes = db::get_page_prizes(&pool, page, page_size)?;
    let total = db::get_total_count_prize(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(PagedData{ data: prizes, total })
    })
}

#[tauri::command]
pub fn get_all_prizes(pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Vec<Prize>>> {
    let prizes = db::get_all_prizes(&pool)?;
    Ok(TauriResponse{
        message: String::new(),
        data: Some(prizes)
    })
}

#[tauri::command]
pub fn get_user(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<UserCard>> {
    Ok(TauriResponse{
        message: String::new(),
        data: Some(db::select_user(&pool, id)?)
    })
}

#[tauri::command]
pub fn get_prize(id: i32, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<Prize>> {
    Ok(TauriResponse{
        message: String::new(),
        data: Some(db::select_prize(&pool, id)?)
    })
}
