use anyhow::{anyhow, Result};
use git2::Repository;

pub async fn create_pull_request() -> Result<()> {
    let _repo = Repository::discover(".")?;

    Err(anyhow!("Nothing to see here."))
}
