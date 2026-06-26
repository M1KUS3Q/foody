use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum DaypartCommand {
    Add { name: String },
    Remove { name: String },
    View { name: String },
    List,
    Assign { mealname: String, dayparts: Vec<String> },
    Unassign { mealname: String, dayparts: Vec<String> },
}

#[tauri::command]
pub async fn daypart(
    state: State<'_, App>,
    command: DaypartCommand,
) -> Result<serde_json::Value, String> {
    match command {
        DaypartCommand::Add { name } => serialize_json(state.add_daypart(&name)).await,
        DaypartCommand::Remove { name } => serialize_json(state.remove_daypart(&name)).await,
        DaypartCommand::View { name } => serialize_json(state.view_daypart(&name)).await,
        DaypartCommand::List => serialize_json(state.list_dayparts()).await,
        DaypartCommand::Assign { mealname, dayparts } => {
            serialize_json(state.assign_dayparts(&mealname, &dayparts)).await
        }
        DaypartCommand::Unassign { mealname, dayparts } => {
            serialize_json(state.unassign_dayparts(&mealname, &dayparts)).await
        }
    }
}
