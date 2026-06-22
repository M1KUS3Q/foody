pub mod app;
pub mod utils;

use std::{env, fs, path::PathBuf, str::FromStr};

use directories::ProjectDirs;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use app::App;

async fn get_db_path() -> anyhow::Result<PathBuf> {
    // 1. Allow DEV override via env var
    if let Ok(url) = env::var("DATABASE_URL") {
        let path = url.strip_prefix("sqlite://").unwrap_or(&url);
        println!("Using database path from DATABASE_URL: {path}...");
        return Ok(PathBuf::from(path));
    }

    // 2. Resolve OS standard app data directory
    let proj_dirs = ProjectDirs::from("com", "miki", "foody")
        .expect("Failed to determine standard application directories");

    let data_dir = proj_dirs.data_dir();

    // 3. Create the directory tree if it does not exist
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }

    Ok(data_dir.join("data.db"))
}

async fn connect(url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(url)?.create_if_missing(true);

    SqlitePool::connect_with(options)
        .await
        .map_err(anyhow::Error::from)
}

/// Initialize the database at the platform-default data directory (CLI/desktop).
pub async fn setup_db() -> anyhow::Result<App> {
    let db_path = get_db_path().await?;
    let db_url = format!("sqlite://{}", db_path.display());
    let pool = connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(App::new(pool))
}

/// Initialize the database at a caller-specified base directory (for Tauri mobile,
/// where the app's sandbox data dir is the only writable location).
pub async fn setup_db_at(base_dir: PathBuf) -> anyhow::Result<App> {
    // Allow env-var override even when a base dir is provided
    if let Ok(url) = env::var("DATABASE_URL") {
        let path = url.strip_prefix("sqlite://").unwrap_or(&url);
        let db_url = format!("sqlite://{}", path);
        println!("Using database path from DATABASE_URL: {path}...");
        let pool = connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        return Ok(App::new(pool));
    }

    if !base_dir.exists() {
        fs::create_dir_all(&base_dir)?;
    }

    let db_path = base_dir.join("data.db");
    let db_url = format!("sqlite://{}", db_path.display());
    println!("Using database at: {}", db_path.display());

    let pool = connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(App::new(pool))
}
