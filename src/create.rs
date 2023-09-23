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
            create_new_branch(&repo, &current_commit, branch_prefix)?
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

fn create_new_branch<'a>(
    repo: &'a Repository,
    commit: &Commit<'a>,
    branch_prefix: Option<String>,
) -> Result<Branch<'a>, git2::Error> {
    // Get commit message summary.
    let summary = match commit.summary() {
        Some(m) => m,
        None => {
            return Err(git2::Error::from_str(
                "Could not get the commit message summary for this commit.",
            ))
        }
    };

    // Transform commit message.
    let mut branch_name = transform(summary);

    if let Some(branch_prefix) = branch_prefix {
        branch_name = format!(
            "{prefix}-{suffix}",
            prefix = branch_prefix,
            suffix = branch_name
        );
    }

    // Create branch.
    repo.branch(&branch_name, commit, false)
}

/// Transform the input string into something that is a valid branch name.
fn transform(summary: &str) -> String {
    // Currently this filters out non-alphabetic characters. A decision made for
    // rapid implementation that discriminates against non-English languages. A
    // better filter would be one based on what characters git and Github don't
    // allow in branch names.
    summary
        .to_lowercase()
        .chars()
        .filter(|c| {
            ('a' <= *c && *c <= 'z')
                || ('A' <= *c && *c <= 'Z')
                || *c == '-'
                || *c == ' '
        })
        .map(|c| if c.is_whitespace() { '-' } else { c })
        // I didn't find any documented limits on Git branch names. I didn't
        // look for documented limits on Github branch names. I did decide there
        // is a practical limit for usability, however I don't know what it is,
        // so this number is arbitrary.
        .take(40)
        .collect()
}
