#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .setup(|app| {
            let url = String::from("https://doc.rust-lang.org/book/");
            app.windows()
                .get("main")
                .unwrap()
                .eval(format!("window.location.replace('{}')", url).as_str())
                .unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
