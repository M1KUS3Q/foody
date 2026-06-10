use crate::{app::App, utils};

impl App {
    pub async fn feedback(&self, content: &str) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO feedback(content) VALUES ($1)", content)
            .execute(&self.pool)
            .await
            .map_err(anyhow::Error::from)?;

        utils::send_feedback(content).await?;

        println!("Thank you for your feedback!");

        Ok(())
    }
}
