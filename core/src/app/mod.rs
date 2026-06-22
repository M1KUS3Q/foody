pub mod category;
pub mod daypart;
pub mod feedback;
pub mod grocery;
pub mod ingredient;
pub mod meal;
pub mod plan;
pub mod recipe;

use sqlx::SqlitePool;

pub struct App {
    pub pool: SqlitePool,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
