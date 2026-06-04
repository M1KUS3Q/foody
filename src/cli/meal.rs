use crate::{app::App, cli::model::MealAction};

pub struct MealRouter;

impl MealRouter {
    pub async fn resolve(app: &mut App, action: &MealAction) -> anyhow::Result<()> {
        match action {
            MealAction::Add { name } => app.add_meal(name).await,
            MealAction::Remove { force, name } => app.remove_meal(name, *force).await,
            MealAction::View { name } => app.view_meal(name).await,
            MealAction::List { filter } => app.list_meals(filter.as_deref()).await,
            MealAction::Rename { name, new_name } => app.rename_meal(name, new_name).await,
        }
    }
}
