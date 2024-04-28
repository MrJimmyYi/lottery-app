use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::api::path::app_data_dir;
use crate::{db, utils};
use crate::db::{AppError, BasicModel, DbPool, Result, TauriResponse};

#[tauri::command]
pub fn get_model_basic(model: String, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<BasicModel>> {
    Ok(TauriResponse{
        message: String::new(),
        data: Some(db::basic::get_model_basic(&pool, model)?)
    })
}

#[tauri::command]
pub fn generate_audiostr(file_name: String, app_handle: tauri::AppHandle) -> Result<TauriResponse<String>> {
    let uploads_dir = app_data_dir(&app_handle.config())
        .ok_or(AppError::ApplicationError("无法获取应用目录".into()))? // 这里修正错误处理
        .join("audio");
    let last_part = file_name.split('.').last().ok_or(AppError::ApplicationError("文件名无扩展名".into()))?;
    let name = format!("{}.{}", utils::generate_random_hash(16), last_part);
    Ok(TauriResponse{
        message: String::new(),
        data: Some(uploads_dir.join(name).to_string_lossy().into_owned())
    })
}


#[tauri::command]
pub fn update_basic_model(basic_model: BasicModel, img_data: Vec<u8>, audio_data: Vec<u8>, pool: tauri::State<'_, DbPool>) -> Result<TauriResponse<i32>> {
    if let Ok(p) = db::basic::get_model_basic(&pool, basic_model.model.clone()) {
        //如果图片需要更新，则需要将原来的图片删除并替换成新的图片
        if !(basic_model.bgImg == p.bgImg) {
            if !p.bgImg.is_empty() {
                fs::remove_file(p.bgImg)?;
            }
            let mut file = File::create(&basic_model.bgImg)?;
            file.write_all(&img_data)?;
        }

        if !(basic_model.audioSrc == p.audioSrc) {
            if !p.audioSrc.is_empty() {
                fs::remove_file(p.audioSrc)?;
            }
            let mut file = File::create(&basic_model.audioSrc)?;
            file.write_all(&audio_data)?;
        }
        db::basic::update_basic_model(&pool, &basic_model)?;
    }
    Ok(TauriResponse{
        message: String::new(),
        data: Some(basic_model.id)
    })
}