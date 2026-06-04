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
