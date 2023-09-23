use git2::{Commit, ObjectType, Repository};

use crate::result::{Error, Result};

pub fn get_selected_commit(repo: &Repository) -> Result<Commit> {
    repo.head()?
        .resolve()?
        .peel(ObjectType::Commit)?
        .into_commit()
        .map_err(|_| Error::NoSelectedCommit)
}
