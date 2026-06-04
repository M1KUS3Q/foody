use crate::app::App;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

impl App {
    pub async fn grocery_plan(&self, name: &str, export: &Option<String>) -> anyhow::Result<()> {
        let plan_id = sqlx::query_scalar!("SELECT id FROM meal_plans WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let items = sqlx::query!(
            "SELECT i.name as ingredient_name, m.name as meal_name
             FROM meal_plan_days mpd
             JOIN meal_plan_day_items mpdi ON mpd.id = mpdi.meal_plan_day_id
             JOIN meals m ON mpdi.meal_id = m.id
             JOIN meal_ingredients mi ON m.id = mi.meal_id
             JOIN ingredients i ON mi.ingredient_id = i.id
             WHERE mpd.meal_plan_id = ?
             ORDER BY i.name ASC, m.name ASC",
            plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut ingredients: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
        for item in items {
            *ingredients
                .entry(item.ingredient_name)
                .or_default()
                .entry(item.meal_name)
                .or_default() += 1;
        }

        let mut output = String::new();
        for (ing, meals) in ingredients {
            let mut meal_strs = Vec::new();
            for (meal, count) in meals {
                if count > 1 {
                    meal_strs.push(format!("{} x{}", meal, count));
                } else {
                    meal_strs.push(meal.clone());
                }
            }
            output.push_str(&format!("{}: {}\n", ing, meal_strs.join(", ")));
        }

        self.handle_grocery_output(output, export)?;
        Ok(())
    }

    pub async fn grocery_meal(&self, name: &str, export: &Option<String>) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let items = sqlx::query!(
            "SELECT i.name as ingredient_name, m.name as meal_name
             FROM meals m
             JOIN meal_ingredients mi ON m.id = mi.meal_id
             JOIN ingredients i ON mi.ingredient_id = i.id
             WHERE m.id = ?
             ORDER BY i.name ASC",
            meal_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut output = String::new();
        for item in items {
            output.push_str(&format!("{}: {}\n", item.ingredient_name, item.meal_name));
        }

        self.handle_grocery_output(output, export)?;
        Ok(())
    }

    fn handle_grocery_output(&self, output: String, export: &Option<String>) -> anyhow::Result<()> {
        if let Some(path) = export {
            let mut file = fs::File::create(path)?;
            file.write_all(output.as_bytes())?;
            println!("Exported grocery list to {}", path);
        } else {
            print!("{}", output);
        }
        Ok(())
    }
}
