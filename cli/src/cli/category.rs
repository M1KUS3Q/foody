use crate::cli::model::GroceryCategoryAction;
use foody_core::app::App;

pub struct CategoryRouter;

impl CategoryRouter {
    pub async fn resolve(app: &mut App, action: &GroceryCategoryAction) -> anyhow::Result<()> {
        match action {
            GroceryCategoryAction::Add { name } => app.add_category(name).await,
            GroceryCategoryAction::Remove { name } => app.remove_category(name).await,
            GroceryCategoryAction::View { name } => {
                let view = app.view_category(name).await?;
                println!("{}", view.to_string_pretty());
                Ok(())
            }
            GroceryCategoryAction::List => {
                let categories = app.list_categories().await?;
                for name in categories {
                    println!("{name}");
                }
                Ok(())
            }
            GroceryCategoryAction::Assign {
                ingredientname,
                categories,
            } => app.assign_categories(ingredientname, categories).await,
            GroceryCategoryAction::Unassign {
                ingredientname,
                categories,
            } => app.unassign_categories(ingredientname, categories).await,
        }
    }
}
