use clap::CommandFactory;
use clap_complete::generate;

use crate::{
    cli::{
        category::CategoryRouter,
        daypart::DaypartRouter,
        grocery::GroceryRouter,
        ingredient::IngredientRouter,
        meal::MealRouter,
        model::{Cli, CliCommandCategory},
        plan::PlanRouter,
        recipe::RecipeRouter,
    },
    upgrade::upgrade_binary,
};
use foody_core::app::App;

pub mod category;
pub mod daypart;
pub mod grocery;
pub mod ingredient;
pub mod meal;
pub mod model;
pub mod plan;
pub mod recipe;

impl Cli {
    pub async fn run(&self, app: &mut App) -> anyhow::Result<()> {
        match &self.command {
            &CliCommandCategory::Upgrade { force } => {
                tokio::task::spawn_blocking(move || upgrade_binary(force)).await??;
                Ok(())
            }
            CliCommandCategory::Completions { shell } => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                generate(*shell, &mut cmd, bin_name, &mut std::io::stdout());
                Ok(())
            }
            CliCommandCategory::Meal { action } => MealRouter::resolve(app, action).await,
            CliCommandCategory::Ingredient { action } => {
                IngredientRouter::resolve(app, action).await
            }
            CliCommandCategory::Daypart { action } => DaypartRouter::resolve(app, action).await,
            CliCommandCategory::GroceryCategory { action } => {
                CategoryRouter::resolve(app, action).await
            }
            CliCommandCategory::Plan { action } => PlanRouter::resolve(app, action).await,
            CliCommandCategory::Grocery { action } => GroceryRouter::resolve(app, action).await,
            CliCommandCategory::Feedback { content } => app.feedback(content).await,
            CliCommandCategory::Recipe { action } => RecipeRouter::resolve(app, action).await,
        }
    }
}
