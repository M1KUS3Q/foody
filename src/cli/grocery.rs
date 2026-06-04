use crate::{app::App, cli::model::GroceryAction};

pub struct GroceryRouter;

impl GroceryRouter {
    pub async fn resolve(app: &mut App, action: &GroceryAction) -> anyhow::Result<()> {
        match action {
            GroceryAction::Plan { name, export } => app.grocery_plan(name, export).await,
            GroceryAction::Meal { name, export } => app.grocery_meal(name, export).await,
        }
    }
}
