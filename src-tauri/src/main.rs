#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{api, Manager};

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .setup(|app| {
            app.windows()
                .get("main")
                .unwrap()
                .eval("window.location.replace('https://doc.rust-lang.org/book/')")
                .unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
