use crate::app::App;

impl App {
    pub async fn add_category(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO categories(name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn remove_category(&self, name: &str) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM categories WHERE name = ?", name)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn view_category(&self, name: &str) -> anyhow::Result<()> {
        let id = sqlx::query_scalar!("SELECT id FROM categories WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let ingredients = sqlx::query_scalar!(
            "SELECT i.name FROM ingredients i JOIN ingredient_categories ic ON ic.ingredient_id = i.id WHERE ic.category_id = ?",
            id
        ).fetch_all(&self.pool).await?;

        println!("Category: {name} (id: {id})");
        println!("Ingredients:");
        for ingredient in ingredients {
            println!("- {ingredient}");
        }

        Ok(())
    }

    pub async fn list_categories(&self) -> anyhow::Result<()> {
        sqlx::query_scalar!("SELECT name FROM categories")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?
            .iter()
            .for_each(|name| println!("{name}"));

        Ok(())
    }

    pub async fn assign_categories(
        &self,
        ingredientname: &str,
        categories: &[String],
    ) -> anyhow::Result<()> {
        let ingredient_id =
            sqlx::query_scalar!("SELECT id FROM ingredients WHERE name = ?", ingredientname)
                .fetch_one(&self.pool)
                .await?;

        for cat in categories {
            let cat_id = sqlx::query_scalar!("SELECT id FROM categories WHERE name = ?", cat)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "INSERT OR IGNORE INTO ingredient_categories (ingredient_id, category_id) VALUES (?, ?)",
                ingredient_id,
                cat_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn unassign_categories(
        &self,
        ingredientname: &str,
        categories: &[String],
    ) -> anyhow::Result<()> {
        let ingredient_id =
            sqlx::query_scalar!("SELECT id FROM ingredients WHERE name = ?", ingredientname)
                .fetch_one(&self.pool)
                .await?;

        for cat in categories {
            let cat_id = sqlx::query_scalar!("SELECT id FROM categories WHERE name = ?", cat)
                .fetch_one(&self.pool)
                .await?;

            sqlx::query!(
                "DELETE FROM ingredient_categories WHERE ingredient_id = ? AND category_id = ?",
                ingredient_id,
                cat_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
