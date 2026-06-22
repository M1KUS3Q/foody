pub async fn send_feedback(content: &str) -> anyhow::Result<()> {
    let feedback_url = env!("FEEDBACK_WEBHOOK_URL");

    let client = reqwest::Client::new();
    let payload = serde_json::json!({ "content": format!("**Received new feedback**:\n{content}")});

    client.post(feedback_url).json(&payload).send().await?;

    Ok(())
}
