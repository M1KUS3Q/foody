mod commands;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Use Tauri's platform-aware app data directory (works on Android/iOS/desktop)
            let data_dir = app.path().app_data_dir().unwrap_or_else(|e| {
                eprintln!("Failed to resolve app data dir: {e}");
                std::path::PathBuf::from(".")
            });

            eprintln!("Initializing database at: {}", data_dir.display());

            let foody_app = tauri::async_runtime::block_on(foody_core::setup_db_at(data_dir))
                .unwrap_or_else(|e| {
                    eprintln!("Database initialization failed: {e}");
                    panic!("Database init failed: {e}");
                });

            app.manage(foody_app);
            eprintln!("Database initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::meal::meal,
            commands::ingredient::ingredient,
            commands::daypart::daypart,
            commands::category::category,
            commands::plan::plan,
            commands::recipe::recipe,
            commands::grocery::grocery,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
