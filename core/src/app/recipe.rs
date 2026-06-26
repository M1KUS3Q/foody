use crate::app::App;

impl App {
    pub async fn set_recipe(&self, meal_name: &str, recipe: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE meals SET recipe = ? WHERE name = ?",
            recipe,
            meal_name
        )
        .execute(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_recipe(&self, meal_name: &str) -> anyhow::Result<Option<String>> {
        let recipe = sqlx::query_scalar!("SELECT recipe FROM meals WHERE name = ?", meal_name)
            .fetch_one(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(recipe)
    }

    pub async fn remove_recipe(&self, meal_name: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE meals SET recipe = NULL WHERE name = ?",
            meal_name
        )
        .execute(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        Ok(())
    }
}
