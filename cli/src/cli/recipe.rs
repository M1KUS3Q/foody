use foody_core::app::App;
use crate::cli::model::RecipeAction;

pub struct RecipeRouter;

impl RecipeRouter {
    pub async fn resolve(app: &mut App, action: &RecipeAction) -> anyhow::Result<()> {
        match action {
            RecipeAction::Set { name, recipe } => app.set_recipe(name, recipe).await,
            RecipeAction::View { name } => {
                let recipe = app.view_recipe(name).await?;
                match recipe {
                    Some(text) => println!("{}", text),
                    None => println!("No recipe set."),
                }
                Ok(())
            }
            RecipeAction::Remove { name } => app.remove_recipe(name).await,
        }
    }
}
