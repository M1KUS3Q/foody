use foody_core::app::App;
use serde::Deserialize;
use tauri::State;

use crate::commands::serialize_json;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum PlanCommand {
    Add { name: String },
    Remove { name: String },
    View { name: String },
    List,
    Rename { name: String, new_name: String },
    Assign {
        planname: String,
        indexname: String,
        daypartname: String,
        mealname: String,
    },
    Unassign {
        planname: String,
        indexname: String,
        daypartname: String,
    },
    Fill { planname: String, days: Option<i64> },
}

#[tauri::command]
pub async fn plan(
    state: State<'_, App>,
    command: PlanCommand,
) -> Result<serde_json::Value, String> {
    match command {
        PlanCommand::Add { name } => serialize_json(state.add_plan(&name)).await,
        PlanCommand::Remove { name } => serialize_json(state.remove_plan(&name)).await,
        PlanCommand::View { name } => serialize_json(state.view_plan(&name)).await,
        PlanCommand::List => serialize_json(state.list_plans()).await,
        PlanCommand::Rename { name, new_name } => {
            serialize_json(state.rename_plan(&name, &new_name)).await
        }
        PlanCommand::Assign {
            planname,
            indexname,
            daypartname,
            mealname,
        } => {
            serialize_json(
                state.assign_plan(&planname, &indexname, &daypartname, &mealname),
            )
            .await
        }
        PlanCommand::Unassign {
            planname,
            indexname,
            daypartname,
        } => {
            serialize_json(state.unassign_plan(&planname, &indexname, &daypartname)).await
        }
        PlanCommand::Fill { planname, days } => {
            serialize_json(state.fill_plan(&planname, days.unwrap_or(7) as usize)).await
        }
    }
}
