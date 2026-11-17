mod commands;
mod db;
mod models;

use commands::DbState;
use std::sync::Mutex;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            std::fs::create_dir_all(&app_data_dir)
                .expect("failed to create app data directory");

            let db_path = app_data_dir.join("job-tracker.sqlite3");
            let conn = db::init_connection(&db_path).expect("failed to initialize database");
            app.manage(DbState(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::list_applications,
            commands::get_application,
            commands::create_application,
            commands::update_application,
            commands::delete_application,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
