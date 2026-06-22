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
            "SELECT i.name as ingredient_name, m.name as meal_name, c.name as category_name
             FROM meal_plan_days mpd
             JOIN meal_plan_day_items mpdi ON mpd.id = mpdi.meal_plan_day_id
             JOIN meals m ON mpdi.meal_id = m.id
             JOIN meal_ingredients mi ON m.id = mi.meal_id
             JOIN ingredients i ON mi.ingredient_id = i.id
             LEFT JOIN ingredient_categories ic ON i.id = ic.ingredient_id
             LEFT JOIN categories c ON ic.category_id = c.id
             WHERE mpd.meal_plan_id = ?
             ORDER BY c.name ASC, i.name ASC, m.name ASC",
            plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut categories: BTreeMap<Option<String>, BTreeMap<String, BTreeMap<String, usize>>> =
            BTreeMap::new();
        for item in items {
            let cat_name: Option<String> = item.category_name;
            *categories
                .entry(cat_name)
                .or_default()
                .entry(item.ingredient_name)
                .or_default()
                .entry(item.meal_name)
                .or_default() += 1;
        }

        let mut output = String::new();
        for (cat, ingredients) in &categories {
            let header = match cat {
                Some(name) => format!("\n[{}]\n", name),
                None => "\n[Uncategorized]\n".into(),
            };
            output.push_str(&header);
            for (ing, meals) in ingredients {
                let mut meal_strs = Vec::new();
                for (meal, count) in meals {
                    if *count > 1 {
                        meal_strs.push(format!("{} x{}", meal, count));
                    } else {
                        meal_strs.push(meal.clone());
                    }
                }
                output.push_str(&format!("  {}: {}\n", ing, meal_strs.join(", ")));
            }
        }

        self.handle_grocery_output(output, export)?;
        Ok(())
    }

    pub async fn grocery_meal(&self, name: &str, export: &Option<String>) -> anyhow::Result<()> {
        let meal_id = sqlx::query_scalar!("SELECT id FROM meals WHERE name = ?", name)
            .fetch_one(&self.pool)
            .await?;

        let items = sqlx::query!(
            "SELECT i.name as ingredient_name, m.name as meal_name, c.name as category_name
             FROM meals m
             JOIN meal_ingredients mi ON m.id = mi.meal_id
             JOIN ingredients i ON mi.ingredient_id = i.id
             LEFT JOIN ingredient_categories ic ON i.id = ic.ingredient_id
             LEFT JOIN categories c ON ic.category_id = c.id
             WHERE m.id = ?
             ORDER BY c.name ASC, i.name ASC",
            meal_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut categories: BTreeMap<Option<String>, Vec<(&str, &str)>> = BTreeMap::new();
        for item in &items {
            let cat_name: Option<String> = item.category_name.clone();
            categories
                .entry(cat_name)
                .or_default()
                .push((&item.ingredient_name, &item.meal_name));
        }

        let mut output = String::new();
        for (cat, ingredients) in &categories {
            let header = match cat {
                Some(name) => format!("\n[{}]\n", name),
                None => "\n[Uncategorized]\n".into(),
            };
            output.push_str(&header);
            for (ing, meal) in ingredients {
                output.push_str(&format!("  {}: {}\n", ing, meal));
            }
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
