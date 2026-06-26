use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum GroceryCommand {
    Plan { name: String },
    Meal { name: String },
}

#[tauri::command]
pub async fn grocery(
    state: State<'_, App>,
    command: GroceryCommand,
) -> Result<serde_json::Value, String> {
    match command {
        GroceryCommand::Plan { name } => serialize_json(state.grocery_plan(&name)).await,
        GroceryCommand::Meal { name } => serialize_json(state.grocery_meal(&name)).await,
    }
}
