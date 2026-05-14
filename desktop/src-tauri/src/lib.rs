mod commands;
mod db;
mod scheduler;

use commands::AppState;
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let db_path = app
                .path()
                .app_data_dir()
                .expect("app data dir not available")
                .join("pulse.db");

            std::fs::create_dir_all(db_path.parent().unwrap()).ok();
            let conn = db::open(&db_path).expect("failed to open database");

            app.manage(AppState {
                db: Arc::new(Mutex::new(conn)),
                scheduler: Arc::new(Scheduler::new()),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_monitors,
            commands::create_monitor,
            commands::update_monitor,
            commands::delete_monitor,
            commands::toggle_active,
            commands::get_heartbeats,
            commands::get_uptime_percentage,
            commands::start_all_monitors,
            commands::pause_monitor,
            commands::resume_monitor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
