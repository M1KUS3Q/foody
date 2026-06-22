use foody_core::app::App;
use crate::cli::model::GroceryCategoryAction;

pub struct CategoryRouter;

impl CategoryRouter {
    pub async fn resolve(app: &mut App, action: &GroceryCategoryAction) -> anyhow::Result<()> {
        match action {
            GroceryCategoryAction::Add { name } => app.add_category(name).await,
            GroceryCategoryAction::Remove { name } => app.remove_category(name).await,
            GroceryCategoryAction::View { name } => app.view_category(name).await,
            GroceryCategoryAction::List => app.list_categories().await,
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
