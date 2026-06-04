use crate::{app::App, cli::model::DaypartAction};

pub struct DaypartRouter;

impl DaypartRouter {
    pub async fn resolve(app: &mut App, action: &DaypartAction) -> anyhow::Result<()> {
        match action {
            DaypartAction::Add { name } => app.add_daypart(name).await,
            DaypartAction::Remove { name } => app.remove_daypart(name).await,
            DaypartAction::View { name } => app.view_daypart(name).await,
            DaypartAction::List => app.list_dayparts().await,
            DaypartAction::Assign { mealname, dayparts } => {
                app.assign_dayparts(mealname, dayparts).await
            }
            DaypartAction::Unassign { mealname, dayparts } => {
                app.unassign_dayparts(mealname, dayparts).await
            }
        }
    }
}
