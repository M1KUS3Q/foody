use foody_core::app::App;
use crate::cli::model::GroceryAction;

pub struct GroceryRouter;

impl GroceryRouter {
    pub async fn resolve(app: &mut App, action: &GroceryAction) -> anyhow::Result<()> {
        match action {
            GroceryAction::Plan { name, export } => {
                let output = app.grocery_plan(name).await?;
                if let Some(path) = export {
                    std::fs::write(path, &output)?;
                    println!("Exported grocery list to {}", path);
                } else {
                    print!("{output}");
                }
                Ok(())
            }
            GroceryAction::Meal { name, export } => {
                let output = app.grocery_meal(name).await?;
                if let Some(path) = export {
                    std::fs::write(path, &output)?;
                    println!("Exported grocery list to {}", path);
                } else {
                    print!("{output}");
                }
                Ok(())
            }
        }
    }
}
