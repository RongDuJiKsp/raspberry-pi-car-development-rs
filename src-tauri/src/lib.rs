#![allow(dead_code)]
mod client;
mod handles;
mod mlib;
mod utils;
use handles::{action, conn};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            conn::connto,
            action::go,
            action::stop,
            action::left,
            action::right,
            action::back,
            action::go_left,
            action::go_right,
            action::back_left,
            action::back_right
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
