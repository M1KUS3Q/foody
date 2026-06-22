use foody_core::app::App;
use crate::cli::model::PlanAction;

pub struct PlanRouter;

impl PlanRouter {
    pub async fn resolve(app: &mut App, action: &PlanAction) -> anyhow::Result<()> {
        match action {
            PlanAction::Add { name } => app.add_plan(name).await,
            PlanAction::Remove { name } => app.remove_plan(name).await,
            PlanAction::View { name } => app.view_plan(name).await,
            PlanAction::List => app.list_plans().await,
            PlanAction::Rename { name, new_name } => app.rename_plan(name, new_name).await,
            PlanAction::Assign {
                planname,
                indexname,
                daypartname,
                mealname,
            } => {
                app.assign_plan(planname, indexname, daypartname, mealname)
                    .await
            }
            PlanAction::Unassign {
                planname,
                indexname,
                daypartname,
            } => app.unassign_plan(planname, indexname, daypartname).await,
            PlanAction::Fill { planname, days } => app.fill_plan(planname, *days).await,
        }
    }
}
