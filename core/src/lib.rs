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
    let proj_dirs = ProjectDirs::from("com", "user", "foody")
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

pub async fn setup_db() -> anyhow::Result<App> {
    let db_path = get_db_path().await?;
    let db_url = format!("sqlite://{}", db_path.display());

    // Ensure database file is created if missing
    let pool = connect(&db_url).await?;

    // Migrations are embedded in the binary at compile time
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(App::new(pool))
}
