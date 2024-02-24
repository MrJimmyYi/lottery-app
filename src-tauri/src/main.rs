// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {

    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let rule = CustomMenuItem::new("rule".to_string(), "规则");
    let data = CustomMenuItem::new("data".to_string(), "数据");
    let submenu = Submenu::new("配置", Menu::new().add_item(rule).add_item(data));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "rule" => {
                    println!("rule");
                }
                "data" => {
                    println!("data");
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
