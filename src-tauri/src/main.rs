#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::File, io::Write};

use tauri::{Manager, Menu};

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            Menu::os_default(&context.package_info().name)
        } else {
            Menu::default()
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
        .on_page_load(|window, payload| {
            let app_dir = window.app_handle().path_resolver().app_dir().unwrap();

            let mut file = File::create(format!("{}state.txt", app_dir.to_str().unwrap())).unwrap();
            match file.write_all(format!("{}", payload.url()).as_bytes()) {
                Ok(()) => println!("url saved"),
                Err(err) => println!("error: {}", &err.to_string()),
            }
        })
        // save dimensions on resize
        .run(context)
        .expect("error while running tauri application");
}
