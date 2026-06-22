use crate::app::App;

impl App {
    pub async fn add_plan(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO meal_plans(name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn remove_plan(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM meal_plans WHERE name = ?", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_plan(&self, name: &str) -> anyhow::Result<()> {
        let id = sqlx::query_scalar!("SELECT id FROM meal_plans WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        println!("{} (id {})", name, id);

        #[derive(sqlx::FromRow)]
        struct PlanItem {
            day_index: i64,
            daypart_name: String,
            meal_name: String,
        }

        let items = sqlx::query_as!(
            PlanItem,
            "SELECT mpd.day_index, dp.name as daypart_name, m.name as meal_name
             FROM meal_plan_days mpd
             JOIN meal_plan_day_items mpdi ON mpd.id = mpdi.meal_plan_day_id
             JOIN dayparts dp ON mpdi.daypart_id = dp.id
             JOIN meals m ON mpdi.meal_id = m.id
             WHERE mpd.meal_plan_id = ?
             ORDER BY mpd.day_index, dp.id",
            id
        )
        .fetch_all(&self.pool)
        .await?;

        for item in items {
            println!(
                "  Day {}: {} - {}",
                item.day_index, item.daypart_name, item.meal_name
            );
        }

        Ok(())
    }

    pub async fn list_plans(&self) -> anyhow::Result<()> {
        sqlx::query_scalar!("SELECT name FROM meal_plans")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?
            .iter()
            .for_each(|name| println!("{name}"));

        Ok(())
    }

    pub async fn rename_plan(&self, name: &str, new_name: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE meal_plans SET name = ? WHERE name = ?",
            new_name,
            name
        )
        .execute(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn assign_plan(
        &self,
        planname: &str,
        indexname: &str,
        daypartname: &str,
        mealname: &str,
    ) -> anyhow::Result<()> {
        let plan_id = sqlx::query_scalar!("SELECT id FROM meal_plans WHERE name = ?", planname)
            .fetch_one(&self.pool)
            .await?;

        let day_index: i64 = indexname.parse()?;

        // ON CONFLICT(meal_plan_id, day_index) requires returning the ID.
        // `query_scalar!` with returning id works in recent SQLite/sqlx
        let day_id = sqlx::query_scalar!(
            "INSERT INTO meal_plan_days (meal_plan_id, day_index) VALUES (?, ?) 
             ON CONFLICT(meal_plan_id, day_index) DO UPDATE SET meal_plan_id=meal_plan_id 
             RETURNING id",
            plan_id,
            day_index
        )
        .fetch_one(&self.pool)
        .await?;

        let daypart_id = sqlx::query_scalar!("SELECT id FROM dayparts WHERE name = ?", daypartname)
            .fetch_one(&self.pool)
            .await?;

        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", mealname)
            .fetch_one(&self.pool)
            .await?;

        sqlx::query!(
            "INSERT INTO meal_plan_day_items (meal_plan_day_id, daypart_id, meal_id) VALUES (?, ?, ?) 
             ON CONFLICT(meal_plan_day_id, daypart_id) DO UPDATE SET meal_id=?",
            day_id,
            daypart_id,
            meal_id,
            meal_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn unassign_plan(
        &self,
        planname: &str,
        indexname: &str,
        daypartname: &str,
    ) -> anyhow::Result<()> {
        let plan_id = sqlx::query_scalar!("SELECT id FROM meal_plans WHERE name = ?", planname)
            .fetch_one(&self.pool)
            .await?;

        let day_index: i64 = indexname.parse()?;

        let daypart_id = sqlx::query_scalar!("SELECT id FROM dayparts WHERE name = ?", daypartname)
            .fetch_one(&self.pool)
            .await?;

        let day_id = sqlx::query_scalar!(
            "SELECT id FROM meal_plan_days WHERE meal_plan_id = ? AND day_index = ?",
            plan_id,
            day_index
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(day_id) = day_id {
            sqlx::query!(
                "DELETE FROM meal_plan_day_items WHERE meal_plan_day_id = ? AND daypart_id = ?",
                day_id,
                daypart_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn fill_plan(&self, planname: &str, days: usize) -> anyhow::Result<()> {
        let plan_id = sqlx::query_scalar!("SELECT id FROM meal_plans WHERE name = ?", planname)
            .fetch_one(&self.pool)
            .await?;

        let dayparts = sqlx::query!("SELECT * FROM dayparts")
            .fetch_all(&self.pool)
            .await?;

        for day_index in 0..days as i64 {
            let day_id = sqlx::query_scalar!(
                "INSERT INTO meal_plan_days (meal_plan_id, day_index) VALUES (?, ?) RETURNING id",
                plan_id,
                day_index
            )
            .fetch_one(&self.pool)
            .await?;

            for dp in &dayparts {
                let meal = sqlx::query_scalar!(
                    "SELECT meal_dayparts.meal_id FROM meal_dayparts
                 WHERE meal_dayparts.daypart_id = ? ORDER BY RANDOM() LIMIT 1",
                    dp.id
                )
                .fetch_optional(&self.pool)
                .await?;

                let Some(meal_id) = meal else {
                    continue;
                };

                sqlx::query!(
                    "INSERT INTO meal_plan_day_items (meal_plan_day_id, daypart_id, meal_id) VALUES (?, ?, ?)",
                    day_id,
                    dp.id,
                    meal_id
                ).execute(&self.pool).await?;
            }
        }

        Ok(())
    }
}
