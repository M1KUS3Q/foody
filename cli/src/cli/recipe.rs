use foody_core::app::App;
use crate::cli::model::RecipeAction;

pub struct RecipeRouter;

impl RecipeRouter {
    pub async fn resolve(app: &mut App, action: &RecipeAction) -> anyhow::Result<()> {
        match action {
            RecipeAction::Set { name, recipe } => app.set_recipe(name, recipe).await,
            RecipeAction::View { name } => app.view_recipe(name).await,
            RecipeAction::Remove { name } => app.remove_recipe(name).await,
        }
    }
}
