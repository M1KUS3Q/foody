use crate::app::App;

pub struct MealView {
    id: i64,
    name: String,
    dayparts: Vec<String>,
    ingredients: Vec<String>,
    recipe: Option<String>,
}

impl MealView {
    pub fn to_string_pretty(&self) -> String {
        let mut s = format!("{} (id {})\n", self.name, self.id);

        if !self.dayparts.is_empty() {
            s.push_str(&format!("Dayparts: {}\n", self.dayparts.join(", ")));
        }

        if !self.ingredients.is_empty() {
            s.push_str("Ingredients:\n");
            for ing in &self.ingredients {
                s.push_str(&format!("- {}\n", ing));
            }
        }

        if let Some(recipe) = &self.recipe {
            s.push_str(&format!("\nRecipe:\n{recipe}\n"));
        }

        s
    }
}

impl App {
    pub async fn add_meal(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO meals(name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn remove_meal(&self, name: &str, force: bool) -> anyhow::Result<()> {
        // TODO: check if meal is used in meal plans and refuse to delete if so, unless --force is used
        if !force {
            let used_by_mealplans = sqlx::query_scalar!(
                "SELECT DISTINCT mp.name
FROM meal_plans mp
JOIN meal_plan_days mpd ON mp.id = mpd.meal_plan_id
JOIN meal_plan_day_items mpdi ON mpd.id = mpdi.meal_plan_day_id
JOIN meals m ON mpdi.meal_id = m.id
WHERE m.name = ?;",
                name
            )
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

            if !used_by_mealplans.is_empty() {
                anyhow::bail!(
                    "Cannot delete meal '{}', it is used by the following meal plans: {}. Use `force` flag to delete anyway.",
                    name,
                    used_by_mealplans.join(", ")
                );
            }
        }

        sqlx::query!("DELETE FROM meals WHERE name = ?", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_meal(&self, name: &str) -> anyhow::Result<()> {
        let id = sqlx::query_scalar!("SELECT m.id FROM meals m WHERE m.name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let dayparts = sqlx::query_scalar!(
            "SELECT d.name FROM dayparts d JOIN meal_dayparts md ON md.daypart_id = d.id WHERE md.meal_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        let ingredients = sqlx::query_scalar!(
            "SELECT ing.name FROM meal_ingredients mi JOIN ingredients ing on mi.ingredient_id = ing.id WHERE mi.meal_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        let recipe = sqlx::query_scalar!("SELECT recipe FROM meals WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        let view = MealView {
            id,
            name: name.into(),
            dayparts,
            ingredients,
            recipe,
        };

        println!("{}", view.to_string_pretty());

        Ok(())
    }

    pub async fn list_meals(&self) -> anyhow::Result<()> {
        sqlx::query_scalar!("SELECT name FROM meals")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?
            .iter()
            .for_each(|name| println!("{name}"));

        Ok(())
    }

    pub async fn rename_meal(&self, name: &str, new_name: &str) -> anyhow::Result<()> {
        sqlx::query!("UPDATE meals SET name = ? WHERE name = ?", new_name, name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }
}
