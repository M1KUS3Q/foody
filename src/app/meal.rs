use crate::app::App;
use cel::Program;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::Json;

#[derive(Serialize, Deserialize)]
pub struct MealView {
    id: i64,
    name: String,
    dayparts: Json<Vec<String>>,
    ingredients: Json<Vec<String>>,
}

impl MealView {
    pub fn to_string_pretty(&self) -> String {
        let mut s = format!("{} (id {})\n", self.name, self.id);

        if !self.dayparts.is_empty() {
            s.push_str(&format!("Dayparts: {}\n", self.dayparts.join(", ")));
        }

        if !self.ingredients.is_empty() {
            s.push_str("Ingredients:\n");
            for ing in &self.ingredients.0 {
                s.push_str(&format!("- {}\n", ing));
            }
        }

        s
    }
}

pub struct MealFilter {
    filter: Program,
}
impl MealFilter {
    pub fn parse(query: &str) -> anyhow::Result<Self> {
        Ok(Self {
            filter: Program::compile(query)?,
        })
    }

    pub fn run(&self, meal: &MealView) -> anyhow::Result<bool> {
        let mut ctx = cel::Context::default();
        ctx.add_variable("id", meal.id)?;
        ctx.add_variable("name", meal.name.clone())?;
        ctx.add_variable("dayparts", meal.dayparts.clone())?;
        ctx.add_variable("ingredients", meal.ingredients.clone())?;

        let res = self.filter.execute(&ctx)?;

        Ok(res == cel::Value::Bool(true))
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
        ).fetch_all(&self.pool).await?.into();

        let ingredients = sqlx::query_scalar!(
            "SELECT ing.name FROM meal_ingredients mi JOIN ingredients ing on mi.ingredient_id = ing.id WHERE mi.meal_id = ?",
            id
        ).fetch_all(&self.pool).await?.into();

        let view = MealView {
            id,
            name: name.into(),
            dayparts,
            ingredients,
        };

        println!("{}", view.to_string_pretty());

        Ok(())
    }

    pub async fn list_meals(&self, filter: Option<&str>) -> anyhow::Result<()> {
        match filter {
            None | Some("") => self.list_meals_unfiltered().await,
            Some(f) => self.list_meals_filtered(f).await,
        }
    }

    async fn list_meals_unfiltered(&self) -> anyhow::Result<()> {
        sqlx::query_scalar!("SELECT name FROM meals")
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)?
            .iter()
            .for_each(|name| println!("{name}"));

        Ok(())
    }
    async fn list_meals_filtered(&self, filter: &str) -> anyhow::Result<()> {
        let meals = sqlx::query_as!(
            MealView,
            r#"SELECT 
                m.id, 
                m.name,
                (SELECT json_group_array(d.name) FROM dayparts d JOIN meal_dayparts md ON md.daypart_id = d.id WHERE md.meal_id = m.id) AS "dayparts!: Json<Vec<String>>",
                (SELECT json_group_array(ing.name) FROM meal_ingredients mi JOIN ingredients ing on mi.ingredient_id = ing.id WHERE mi.meal_id = m.id) AS "ingredients!: Json<Vec<String>>"
            FROM meals m"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::from)?;

        let meal_filter = MealFilter::parse(filter)?;

        meals
            .into_iter()
            .filter(|meal| meal_filter.run(meal).unwrap_or(false))
            .for_each(|meal| println!("{}", meal.name));

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
