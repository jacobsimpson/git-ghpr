use git2::{Commit, ObjectType, Repository};

pub fn get_selected_commit(
    repo: &Repository,
) -> core::result::Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("No currently selected commit."))
}
