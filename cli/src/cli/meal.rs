use foody_core::app::App;
use crate::cli::model::MealAction;

pub struct MealRouter;

impl MealRouter {
    pub async fn resolve(app: &mut App, action: &MealAction) -> anyhow::Result<()> {
        match action {
            MealAction::Add { name } => app.add_meal(name).await,
            MealAction::Remove { force, name } => app.remove_meal(name, *force).await,
            MealAction::View { name } => {
                let view = app.view_meal(name).await?;
                println!("{}", view.to_string_pretty());
                Ok(())
            }
            MealAction::List => {
                let meals = app.list_meals().await?;
                for name in meals {
                    println!("{name}");
                }
                Ok(())
            }
            MealAction::Rename { name, new_name } => app.rename_meal(name, new_name).await,
        }
    }
}
