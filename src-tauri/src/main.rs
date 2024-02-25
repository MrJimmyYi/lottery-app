#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {

    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let rule = CustomMenuItem::new("pageManage".to_string(), "Manage");
    let data = CustomMenuItem::new("pageLckUsers".to_string(), "Prize List");
    let settings = Submenu::new("Settings", Menu::new().add_item(rule).add_item(data));
    let weeding = CustomMenuItem::new("pageWeeding".to_string(), "Wedding");
    let model = Submenu::new("model", Menu::new().add_item(weeding));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(model)
        .add_submenu(settings);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            println!("{}",event_name );
            if let window = event.window() {
                window.emit("navigate", event_name).expect("Failed to emit navigate event");
            }

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
