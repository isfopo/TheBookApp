#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs::File,
    io::{Read, Write},
};

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
            let app_dir = app.path_resolver().app_dir().unwrap();

            let mut file = match File::open(app_dir.join("state.txt")) {
                Ok(file) => file,
                Err(_) => File::create(app_dir.join("state.txt")).unwrap(),
            };

            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let url = String::from(if contents.is_empty() {
                "https://doc.rust-lang.org/book/"
            } else {
                &contents
            });

            app.windows()
                .get("main")
                .unwrap()
                .eval(format!("window.location.replace('{}')", url).as_str())
                .unwrap();
            Ok(())
        })
        .on_page_load(|window, payload| {
            let app_dir = window.app_handle().path_resolver().app_dir().unwrap();

            let mut file = File::create(app_dir.join("state.txt")).unwrap();
            match file.write_all(format!("{}", payload.url()).as_bytes()) {
                Ok(()) => (),
                Err(err) => println!("error: {}", &err.to_string()),
            }
        })
        // save dimensions on resize
        .run(context)
        .expect("error while running tauri application");
}
