#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // https://github.com/tauri-apps/wry/issues/415
    env::set_var( "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--proxy-server 127.0.0.1:12345" );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
