use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum RecipeCommand {
    Set { meal_name: String, recipe: String },
    View { meal_name: String },
    Remove { meal_name: String },
}

#[tauri::command]
pub async fn recipe(
    state: State<'_, App>,
    command: RecipeCommand,
) -> Result<serde_json::Value, String> {
    match command {
        RecipeCommand::Set { meal_name, recipe } => {
            serialize_json(state.set_recipe(&meal_name, &recipe)).await
        }
        RecipeCommand::View { meal_name } => serialize_json(state.view_recipe(&meal_name)).await,
        RecipeCommand::Remove { meal_name } => {
            serialize_json(state.remove_recipe(&meal_name)).await
        }
    }
}
