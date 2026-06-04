use crate::{app::App, cli::model::IngredientAction};

pub struct IngredientRouter;

impl IngredientRouter {
    pub async fn resolve(app: &mut App, action: &IngredientAction) -> anyhow::Result<()> {
        match action {
            IngredientAction::Add { name } => app.add_ingredient(name).await,
            IngredientAction::Remove { force, name } => app.remove_ingredient(name, *force).await,
            IngredientAction::View { name } => app.view_ingredient(name).await,
            IngredientAction::List => app.list_ingredients().await,
            IngredientAction::Rename { name, new_name } => {
                app.rename_ingredient(name, new_name).await
            }
            IngredientAction::Assign {
                mealname,
                ingredients,
            } => app.assign_ingredients(mealname, ingredients).await,
            IngredientAction::Unassign {
                mealname,
                ingredients,
            } => app.unassign_ingredients(mealname, ingredients).await,
        }
    }
}
