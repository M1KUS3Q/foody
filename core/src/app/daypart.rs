use crate::app::App;

impl App {
    pub async fn add_daypart(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO dayparts(name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn remove_daypart(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM dayparts WHERE name = ?", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_daypart(&self, name: &str) -> anyhow::Result<()> {
        let id = sqlx::query_scalar!("SELECT id FROM dayparts WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let meals= sqlx::query_scalar!(
            "SELECT m.name FROM meals m JOIN meal_dayparts md ON md.meal_id = m.id WHERE md.daypart_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        println!("Daypart: {name} (id: {id})");
        println!("Meals:");
        for meal in meals {
            println!("- {meal}");
        }

        Ok(())
    }

    pub async fn list_dayparts(&self) -> anyhow::Result<()> {
        sqlx::query_scalar!("SELECT name FROM dayparts")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?
            .iter()
            .for_each(|name| println!("{name}"));

        Ok(())
    }

    pub async fn assign_dayparts(&self, mealname: &str, dayparts: &[String]) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", mealname)
            .fetch_one(&self.pool)
            .await?;

        for dp in dayparts {
            let dp_id = sqlx::query_scalar!("SELECT id FROM dayparts WHERE name = ?", dp)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "INSERT OR IGNORE INTO meal_dayparts (meal_id, daypart_id) VALUES (?, ?)",
                meal_id,
                dp_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn unassign_dayparts(
        &self,
        mealname: &str,
        dayparts: &[String],
    ) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", mealname)
            .fetch_one(&self.pool)
            .await?;

        for dp in dayparts {
            let dp_id = sqlx::query_scalar!("SELECT id FROM dayparts WHERE name = ?", dp)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "DELETE FROM meal_dayparts WHERE meal_id = ? AND daypart_id = ?",
                meal_id,
                dp_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
