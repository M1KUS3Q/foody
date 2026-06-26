use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum MealCommand {
    Add { name: String },
    Remove { name: String, force: Option<bool> },
    View { name: String },
    List,
    Rename { name: String, new_name: String },
}

#[tauri::command]
pub async fn meal(
    state: State<'_, App>,
    command: MealCommand,
) -> Result<serde_json::Value, String> {
    match command {
        MealCommand::Add { name } => serialize_json(state.add_meal(&name)).await,
        MealCommand::Remove { name, force } => {
            serialize_json(state.remove_meal(&name, force.unwrap_or(false))).await
        }
        MealCommand::View { name } => serialize_json(state.view_meal(&name)).await,
        MealCommand::List => serialize_json(state.list_meals()).await,
        MealCommand::Rename { name, new_name } => {
            serialize_json(state.rename_meal(&name, &new_name)).await
        }
    }
}
