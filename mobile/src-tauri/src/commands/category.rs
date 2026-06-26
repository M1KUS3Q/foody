use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum CategoryCommand {
    Add {
        name: String,
    },
    Remove {
        name: String,
    },
    View {
        name: String,
    },
    List,
    Assign {
        ingredientname: String,
        categories: Vec<String>,
    },
    Unassign {
        ingredientname: String,
        categories: Vec<String>,
    },
}

#[tauri::command]
pub async fn category(
    state: State<'_, App>,
    command: CategoryCommand,
) -> Result<serde_json::Value, String> {
    match command {
        CategoryCommand::Add { name } => serialize_json(state.add_category(&name)).await,
        CategoryCommand::Remove { name } => serialize_json(state.remove_category(&name)).await,
        CategoryCommand::View { name } => serialize_json(state.view_category(&name)).await,
        CategoryCommand::List => serialize_json(state.list_categories()).await,
        CategoryCommand::Assign {
            ingredientname,
            categories,
        } => serialize_json(state.assign_categories(&ingredientname, &categories)).await,
        CategoryCommand::Unassign {
            ingredientname,
            categories,
        } => serialize_json(state.unassign_categories(&ingredientname, &categories)).await,
    }
}
