use crate::common::get_selected_commit;
use anyhow::Result;
use git2::Repository;
use tracing::info;

pub async fn create_pull_request() -> Result<()> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    let current_commit = get_selected_commit(&repo)?;
    info!("Current commit = {}", current_commit.id());

    info!("and some more stuff.");

    println!("Created pull request.");
    Ok(())
}
