use foody_core::app::App;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn add_meal(state: tauri::State<'_, App>, name: String) -> Result<(), String> {
    state.add_meal(&name).await.map_err(|e| e.to_string())
}

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
        .invoke_handler(tauri::generate_handler![greet, add_meal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
