pub mod daypart;
pub mod grocery;
pub mod ingredient;
pub mod meal;
pub mod plan;

use sqlx::SqlitePool;

pub struct App {
    pub pool: SqlitePool,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
