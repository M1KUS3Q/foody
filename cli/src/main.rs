mod cli;
mod upgrade;

use clap::Parser;
use cli::model::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env only in development, ignore failures in release
    let _ = dotenv::dotenv();

    let mut app = foody_core::setup_db().await?;
    let cli = Cli::parse();
    cli.run(&mut app).await
}
