use anyhow::{anyhow, Result};
use git2::Repository;
use tracing::info;

pub async fn create_pull_request() -> Result<()> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    Err(anyhow!("Nothing to see here."))
}
