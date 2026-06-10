use self_update::cargo_crate_version;

pub fn upgrade_binary(force: bool) -> anyhow::Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("M1KUS3Q")
        .repo_name("foody")
        .bin_name("foody")
        .show_download_progress(true)
        .no_confirm(force)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: v{}", status.version());
    Ok(())
}

pub async fn send_feedback(content: &str) -> anyhow::Result<()> {
    let feedback_url = env!("FEEDBACK_WEBHOOK_URL");

    let client = reqwest::Client::new();
    let payload = serde_json::json!({ "content": format!("**Received new feedback**:\n{content}")});

    client.post(feedback_url).json(&payload).send().await?;

    Ok(())
}
