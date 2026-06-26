use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum IngredientCommand {
    Add { name: String },
    Remove { name: String, force: Option<bool> },
    View { name: String },
    List,
    Rename { name: String, new_name: String },
    Assign { mealname: String, ingredients: Vec<String> },
    Unassign { mealname: String, ingredients: Vec<String> },
}

#[tauri::command]
pub async fn ingredient(
    state: State<'_, App>,
    command: IngredientCommand,
) -> Result<serde_json::Value, String> {
    match command {
        IngredientCommand::Add { name } => serialize_json(state.add_ingredient(&name)).await,
        IngredientCommand::Remove { name, force } => {
            serialize_json(state.remove_ingredient(&name, force.unwrap_or(false))).await
        }
        IngredientCommand::View { name } => serialize_json(state.view_ingredient(&name)).await,
        IngredientCommand::List => serialize_json(state.list_ingredients()).await,
        IngredientCommand::Rename { name, new_name } => {
            serialize_json(state.rename_ingredient(&name, &new_name)).await
        }
        IngredientCommand::Assign {
            mealname,
            ingredients,
        } => serialize_json(state.assign_ingredients(&mealname, &ingredients)).await,
        IngredientCommand::Unassign {
            mealname,
            ingredients,
        } => {
            serialize_json(state.unassign_ingredients(&mealname, &ingredients)).await
        }
    }
}
