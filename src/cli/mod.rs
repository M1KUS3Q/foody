use clap::CommandFactory;
use clap_complete::generate;

use crate::{
    app::App,
    cli::{
        category::CategoryRouter,
        daypart::DaypartRouter,
        grocery::GroceryRouter,
        ingredient::IngredientRouter,
        meal::MealRouter,
        model::{Category, Cli},
        plan::PlanRouter,
        recipe::RecipeRouter,
    },
    utils::upgrade_binary,
};

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
            &Category::Upgrade { force } => {
                tokio::task::spawn_blocking(move || upgrade_binary(force)).await??;
                Ok(())
            }
            Category::Completions { shell } => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                generate(*shell, &mut cmd, bin_name, &mut std::io::stdout());
                Ok(())
            }
            Category::Meal { action } => MealRouter::resolve(app, action).await,
            Category::Ingredient { action } => IngredientRouter::resolve(app, action).await,
            Category::Daypart { action } => DaypartRouter::resolve(app, action).await,
            Category::GroceryCategory { action } => CategoryRouter::resolve(app, action).await,
            Category::Plan { action } => PlanRouter::resolve(app, action).await,
            Category::Grocery { action } => GroceryRouter::resolve(app, action).await,
            Category::Feedback { content } => app.feedback(content).await,
            Category::Recipe { action } => RecipeRouter::resolve(app, action).await,
        }
    }
}
