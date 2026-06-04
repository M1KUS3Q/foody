use clap::CommandFactory;
use clap_complete::generate;

use crate::{
    app::App,
    cli::{
        daypart::DaypartRouter,
        grocery::GroceryRouter,
        ingredient::IngredientRouter,
        meal::MealRouter,
        model::{Category, Cli},
        plan::PlanRouter,
    },
};

pub mod daypart;
pub mod grocery;
pub mod ingredient;
pub mod meal;
pub mod model;
pub mod plan;

impl Cli {
    pub async fn run(&self, app: &mut App) -> anyhow::Result<()> {
        match &self.command {
            Category::Completions { shell } => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();
                generate(*shell, &mut cmd, bin_name, &mut std::io::stdout());
                Ok(())
            }
            Category::Meal { action } => MealRouter::resolve(app, action).await,
            Category::Ingredient { action } => IngredientRouter::resolve(app, action).await,
            Category::Daypart { action } => DaypartRouter::resolve(app, action).await,
            Category::Plan { action } => PlanRouter::resolve(app, action).await,
            Category::Grocery { action } => GroceryRouter::resolve(app, action).await,
        }
    }
}
