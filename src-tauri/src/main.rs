#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod db;
mod utils;
mod commands;
use db::*;
use commands::*;

use std::env::temp_dir;
use tauri::{Builder, Manager};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::api::path::app_data_dir;
use crate::commands::prize::*;
use crate::commands::user::*;
use crate::commands::winner::*;
use crate::commands::basic::*;

fn main() {

    let pool = match init_app() {
        Some(p) => p,
        None => std::process::exit(1),
    };

    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let rule = CustomMenuItem::new("Manage".to_string(), "后台管理");
    let data = CustomMenuItem::new("Winners".to_string(), "中奖人员");
    let settings = Submenu::new("设置", Menu::new().add_item(rule).add_item(data));
    let weeding = CustomMenuItem::new("Wedding".to_string(), "婚礼模式");
    let model = Submenu::new("模式", Menu::new().add_item(weeding));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(model)
        .add_submenu(settings);

    tauri::Builder::default()
        .setup(|app|{
            let app_dir = app_data_dir(&app.app_handle().config()).ok_or_else(|| "无法获取应用目录".to_string())?;
            let imgs_dir = app_dir.join("img");
            std::fs::create_dir_all(&imgs_dir).map_err(|e| e.to_string())?;

            let audio_dir = app_dir.join("audio");
            std::fs::create_dir_all(&audio_dir).map_err(|e| e.to_string())?;


            // 获取系统的临时目录
            let temp_dir = temp_dir();
            // 在临时目录下创建一个新的目录来存放上传的文件，以避免潜在的命名冲突
            let uploads_dir = temp_dir.join("file");
            // 尝试创建目录（如果它还不存在）
            std::fs::create_dir_all(&uploads_dir).map_err(|e| e.to_string())?;
            Ok(())
        })
        // 将数据库连接池存储为应用状态
        .manage(pool)
        .menu(menu)
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            if let window = event.window() {
                window.emit("navigate", event_name).expect("Failed to emit navigate event");
            }
        })
        .invoke_handler(tauri::generate_handler![get_all_winners,get_page_winners,create_winners,
            update_prize_count, get_all_prizes,
            get_all_users, update_prize, create_prize, get_prize, get_page_prizes, delete_prize, update_user,
            get_user, generate_imgstr,create_user,get_page_users,batch_excel_operate,download_template_excel,delete_user,
            get_model_basic, generate_audiostr, update_basic_model])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
