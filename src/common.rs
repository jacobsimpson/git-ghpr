use git2::{Commit, ObjectType, Repository};
use tracing::info;

use crate::result::{Error, Result};

pub fn get_selected_commit(repo: &Repository) -> Result<Commit> {
    let current_commit = repo
        .head()?
        .resolve()?
        .peel(ObjectType::Commit)?
        .into_commit()
        .map_err(|_| Error::NoSelectedCommit)?;

    info!("Current commit = {}", current_commit.id());
    Ok(current_commit)
}
