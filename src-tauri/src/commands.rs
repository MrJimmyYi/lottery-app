use std::env;
use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use crate::db;
use crate::db::DbPool;
use crate::model::{AppError, PagedUserResponse, UserCard};
use crate::utils::generate_random_hash;
use thiserror::Error;
use std::fs;

pub type Result<T> = std::result::Result<T, AppError>;

#[tauri::command]
pub fn create_user(user: UserCard, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<String> {
    let mut file = File::create(&user.img)?;
    file.write_all(&img_data)?;
    db::create_user(&pool, &user)?;
    Ok("User created successfully".into())
}

#[tauri::command]
pub fn update_user(user: UserCard, img_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<i32> {
    if let Ok(userCard) = db::select_user(&pool, user.id) {
        db::edit_user(&pool, &user)?;
        //如果图片需要更新，则需要将原来的图片删除并替换成新的图片
        if !(user.img == userCard.img) {
            fs::remove_file(userCard.img)?;
            let mut file = File::create(&user.img)?;
            file.write_all(&img_data)?;
        }
    }
    Ok(user.id)
}

#[tauri::command]
pub fn delete_user(id: i32, pool: tauri::State<'_, DbPool>) -> Result<i32> {
    if let Ok(userCard) = db::select_user(&pool, id) {
        db::delete_user(&pool, userCard.id)?;
        fs::remove_file(userCard.img)?;
    }
    Ok(id)
}

#[tauri::command]
pub fn generate_imgstr(file_name: String, app_handle: tauri::AppHandle) -> Result<String> {
    let uploads_dir = app_data_dir(&app_handle.config())
        .ok_or(AppError::ApplicationError("无法获取应用目录".into()))? // 这里修正错误处理
        .join("img");
    let last_part = file_name.split('.').last().ok_or(AppError::ApplicationError("文件名无扩展名".into()))?;
    let name = format!("{}.{}", generate_random_hash(16), last_part);
    Ok(uploads_dir.join(name).to_string_lossy().into_owned())
}


#[tauri::command]
pub fn batch_excel_operate(pool: tauri::State<'_, DbPool>, file_name: String, file_content: Vec<u8>, app_handle: tauri::AppHandle) -> Result<String> {
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

    Ok(temp_file_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn download_template_excel(file_name: String) -> Result<Vec<u8>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(format!("{}.xlsx", file_name));
    let buffer = std::fs::read(&path)?;
    Ok(buffer)
}

#[tauri::command]
pub fn get_page_users(pool: tauri::State<'_, DbPool>, page: usize, page_size: usize) -> Result<PagedUserResponse> {
    let users = db::get_page_users(&pool, page, page_size)?;
    let total = db::get_total_users_count(&pool)?;
    Ok(PagedUserResponse { users, total })
}

#[tauri::command]
pub fn get_user(id: i32, pool: tauri::State<'_, DbPool>) -> Result<UserCard> {
    db::select_user(&pool, id)
}
