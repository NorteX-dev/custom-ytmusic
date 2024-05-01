#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::GlobalShortcutManager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let webview = app.get_window("main").unwrap();
            let _ = webview.eval("window.location.replace('https://music.youtube.com')");
            let webview1 = webview.clone();
            let webview2 = webview.clone();
            let webview3 = webview.clone();
            let _ = app.global_shortcut_manager().register("F21", move || {
                let _ = webview1.eval("document.querySelector('#play-pause-button').click()");
            });
            let _ = app.global_shortcut_manager().register("F22", move || {
                let _ = webview2.eval("document.querySelector('.previous-button').click()");
            });
            let _ = app.global_shortcut_manager().register("F23", move || {
                let _ = webview3.eval("document.querySelector('.next-button').click()");
            });
            Ok(())
        })
    
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
