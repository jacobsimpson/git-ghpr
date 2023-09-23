use git2::Branch;
use git2::BranchType;
use git2::Commit;
use git2::Repository;
use tracing::{error, info};

use crate::common::get_selected_commit;
use crate::result::Error;
use crate::result::Message;
use crate::result::Result;

pub async fn create_pull_request() -> Result<Message> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    let current_commit = get_selected_commit(&repo)?;
    info!("Current commit = {}", current_commit.id());

    let _current_branch = match get_branch_for_commit(&repo, &current_commit)? {
        Some(b) => b,
        None => {
            info!("No existing branch, creating a new one.");
            return Err(Error::UnableToCreateBranch {
                branch_name: "not-yet".to_string(),
                base_commit: current_commit.id().to_string(),
            });
        }
    };

    Ok(Message::Empty)
}

fn get_branch_for_commit<'a>(
    repo: &'a Repository,
    commit: &Commit<'a>,
) -> Result<Option<Branch<'a>>, git2::Error> {
    let branches = repo.branches(Some(BranchType::Local))?;

    for branch in branches {
        match branch {
            Ok((branch, _branch_type)) => {
                let branch_commit = branch.get().peel_to_commit().unwrap();
                if branch_commit.id() == commit.id() {
                    return Ok(Some(branch));
                }
            }
            Err(e) => error!("Couldn't list branch: {:?}", e),
        };
    }

    Ok(None)
}
