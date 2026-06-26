use serde::Serialize;

use crate::app::App;

#[derive(Serialize)]
pub struct IngredientView {
    pub id: i64,
    pub name: String,
    pub meals: Vec<String>,
    pub categories: Vec<String>,
}

impl IngredientView {
    pub fn to_string_pretty(&self) -> String {
        let mut s = format!("{} (id {})\n", self.name, self.id);

        if !self.categories.is_empty() {
            s.push_str(&format!("Categories: {}\n", self.categories.join(", ")));
        }

        if !self.meals.is_empty() {
            s.push_str("Used in meals:\n");
            for meal in &self.meals {
                s.push_str(&format!("- {}\n", meal));
            }
        }

        s
    }
}

impl App {
    pub async fn add_ingredient(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO ingredients(name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn remove_ingredient(&self, name: &str, force: bool) -> anyhow::Result<()> {
        if !force {
            let used_by_meals = sqlx::query_scalar!(
                "SELECT m.name
FROM meals m
JOIN meal_ingredients mi ON m.id = mi.meal_id
JOIN ingredients i ON mi.ingredient_id = i.id
WHERE i.name = ?;",
                name
            )
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

            if !used_by_meals.is_empty() {
                anyhow::bail!(
                    "Cannot delete ingredient '{}', it is used by the following meals: {}. Use `force` flag to delete anyway.",
                    name,
                    used_by_meals.join(", ")
                );
            }
        }

        sqlx::query!("DELETE FROM ingredients WHERE name = ?", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_ingredient(&self, name: &str) -> anyhow::Result<IngredientView> {
        let id = sqlx::query_scalar!("SELECT id FROM ingredients WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let meals = sqlx::query_scalar!(
            "SELECT m.name FROM meal_ingredients mi JOIN meals m on mi.meal_id = m.id WHERE mi.ingredient_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        let categories = sqlx::query_scalar!(
            "SELECT c.name FROM categories c JOIN ingredient_categories ic ON ic.category_id = c.id WHERE ic.ingredient_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        Ok(IngredientView {
            id,
            name: name.into(),
            meals,
            categories,
        })
    }

    pub async fn list_ingredients(&self) -> anyhow::Result<Vec<String>> {
        sqlx::query_scalar!("SELECT name FROM ingredients")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn rename_ingredient(&self, name: &str, new_name: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE ingredients SET name = ? WHERE name = ?",
            new_name,
            name
        )
        .execute(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn assign_ingredients(
        &self,
        mealname: &str,
        ingredients: &[String],
    ) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", mealname)
            .fetch_one(&self.pool)
            .await?;

        for ing in ingredients {
            let ing_id = sqlx::query_scalar!("SELECT id FROM ingredients WHERE name = ?", ing)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "INSERT OR IGNORE INTO meal_ingredients (meal_id, ingredient_id) VALUES (?, ?)",
                meal_id,
                ing_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn unassign_ingredients(
        &self,
        mealname: &str,
        ingredients: &[String],
    ) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", mealname)
            .fetch_one(&self.pool)
            .await?;

        for ing in ingredients {
            let ing_id = sqlx::query_scalar!("SELECT id FROM ingredients WHERE name = ?", ing)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "DELETE FROM meal_ingredients WHERE meal_id = ? AND ingredient_id = ?",
                meal_id,
                ing_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
