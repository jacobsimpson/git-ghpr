use git2::Branch;
use git2::BranchType;
use git2::Commit;
use git2::Repository;
use std::collections::HashMap;
use tracing::{error, info};

use crate::common::get_selected_commit;
use crate::result::Error;
use crate::result::Message;
use crate::result::Result;

/// Creates a pull request for the current commit. This is a safe operation, it
/// will do it's best to detect the current state of the repository and Github,
/// and fill in the missing pieces, or return a useful error message.
/// * Check if there is a remote for the repository.
/// * Find the current commit.
/// * Find the base branch.
/// * Check the base branch is remote.
/// - Check the base branch remote is up to date.
/// - Check the base branch is main or there is a base branch PR.
/// * Find the branch for the current commit.
/// * Create a branch if one does not exist.
/// - Push the branch upstream if necessary, possibly force push.
/// - Check if there is a PR for this branch.
/// - Create a PR for this branch.
pub async fn create_pull_request(
    branch_name_template: &str,
    branch_name_parameters: &HashMap<String, String>,
) -> Result<Message> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    check_has_remote(&repo)?;

    let current_commit = get_selected_commit(&repo)?;

    let base_branch = find_base_branch(&repo, &current_commit)?;

    check_branch_has_remote(&base_branch)?;

    let current_branch = get_or_create_branch(
        &repo,
        &current_commit,
        branch_name_template,
        branch_name_parameters,
    )?;

    if let Err(e) = current_branch.upstream() {
        if e.code() != git2::ErrorCode::NotFound {
            return Err(Error::Generic);
        }
    }

    Ok(Message::Empty)
}

fn check_branch_has_remote(branch: &Branch<'a>) -> Result<()> {
    if let Err(e) = branch.upstream() {
        if e.code() == git2::ErrorCode::NotFound {
            return Err(Error::NoRemoteBranch(branch.name()));
        }
        return Err(Error::Generic);
    }
    Ok(())
}

fn check_has_remote<'a>(repo: &'a Repository) -> Result<()> {
    let remotes = repo.remotes()?;
    if remotes.len() == 0 {
        return Err(Error::NoRemote);
    }
    Ok(())
}

fn get_or_create_branch<'a>(
    repo: &'a Repository,
    current_commit: &Commit<'a>,
    branch_name_template: &str,
    branch_name_parameters: &HashMap<String, String>,
) -> Result<Branch<'a>> {
    let current_branch = match get_branch_for_commit(&repo, &current_commit)? {
        Some(b) => b,
        None => {
            info!("No existing branch, creating a new one.");
            create_new_branch(
                &repo,
                &current_commit,
                branch_name_template,
                branch_name_parameters,
            )?
        }
    };
    Ok(current_branch)
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
    branch_name_template: &str,
    branch_name_parameters: &HashMap<String, String>,
) -> Result<Branch<'a>, Error> {
    // Get commit message summary.
    let summary = match commit.summary() {
        Some(m) => m,
        None => return Err(Error::NoCommitMessage),
    };

    let branch_name = generate_branch_name(
        branch_name_template,
        branch_name_parameters,
        summary,
    )?;

    // Create branch.
    let branch = repo.branch(&branch_name, commit, false).map_err(|_e| {
        Error::UnableToCreateBranch {
            branch_name: branch_name.clone(),
            base_commit: commit.id().to_string(),
        }
    })?;

    // Setting `head` like this, with `refs/heads/XYZ`, is what sets the current
    // current branch for `git` commands. However, doing it this way means that
    // `libgit2` doesn't recognize it as a branch for `is_head` or
    // `symbolic_target`.
    repo.set_head(&format!("refs/heads/{branch_name}"))
        .map_err(|_e| Error::UnableToSelectBranch(branch_name))?;

    Ok(branch)
}

fn generate_branch_name(
    branch_name_template: &str,
    branch_name_parameters: &HashMap<String, String>,
    summary: &str,
) -> Result<String, Error> {
    // Process the summary to something that can be used as a branch name.
    let summary = transform(summary);

    use tera::{Context, Tera};
    let mut context = Context::new();
    context.insert("summary", &summary);
    for (k, v) in branch_name_parameters {
        context.insert(k, &v);
    }
    // "{{ summary ~ '-' }}"
    let branch_name = match Tera::one_off(branch_name_template, &context, true)
    {
        Ok(b) => b,
        Err(e) => {
            return Err(match get_missing_variable(&e) {
                Some(name) => Error::MissingBranchParameter(name),
                None => Error::BranchTemplateMalformed(e.to_string()),
            })
        }
    };

    // if let Some(branch_prefix) = branch_prefix {
    //     branch_name = format!(
    //         "{prefix}-{suffix}",
    //         prefix = branch_prefix,
    //         suffix = branch_name
    //     );
    // }

    Ok(branch_name)
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
                || ('0' <= *c && *c <= '9')
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

/// Given a Tera error message, convert it into the name of the missing
/// variable. If there is a variable in the template that is missing, this is
/// the only way I was able to find to detect the name of the variable.
fn get_missing_variable<T>(e: &T) -> Option<String>
where
    T: std::error::Error,
{
    let e = e.source();
    if e.is_none() {
        return None;
    }

    let e = e.unwrap();

    let e = match e.downcast_ref::<tera::Error>() {
        Some(e) => e,
        None => return None,
    };

    let m = match &e.kind {
        tera::ErrorKind::Msg(m) => m,
        _ => return None,
    };

    if !m.starts_with("Variable `")
        || !m.contains("` not found in context while rendering ")
    {
        return None;
    }

    let start = m.find('`').unwrap();
    let end = m.rfind('`').unwrap();
    let variable = m[start + 1..end].to_string();

    Some(variable)
}

fn get_main_branch_commit<'a>(
    repo: &'a Repository,
) -> Result<(Commit, String)> {
    // There is a `mainBranch` property in the branchless section of a
    // configured git repo. I would prefer to take the main branch name from
    // there, so behavior is consistent with branchless.
    let branches = repo.branches(Some(BranchType::Local))?;

    for branch in branches {
        match branch {
            Ok((branch, _branch_type)) => {
                if branch.name()? == Some("main") {
                    return Ok((
                        branch.get().peel_to_commit().unwrap(),
                        "main".to_string(),
                    ));
                } else if branch.name()? == Some("master") {
                    return Ok((
                        branch.get().peel_to_commit().unwrap(),
                        "master".to_string(),
                    ));
                }
            }
            Err(e) => println!("Couldn't list branch: {:?}", e),
        };
    }

    Err(Error::UnknownMainBranch)
}

fn find_base_branch<'a>(
    repo: &'a Repository,
    current_commit: &'a Commit,
) -> Result<String> {
    let (main_commit, main_branch_name) = get_main_branch_commit(repo)?;

    let merge_base = repo.merge_base(main_commit.id(), current_commit.id())?;

    let mut commit = current_commit.clone();

    while commit.id() != merge_base {
        if commit.parents().len() == 0 {
            return Err(Error::NoBaseBranch);
        }

        if commit.parents().len() > 1 {
            // This error message would work better with the branch name when it is
            // available. I didn't do it at the time because of time constraints.
            return Err(Error::MultipleParentCommits(commit.id().to_string()));
        }

        let parent_commit = commit.parents().nth(0).unwrap();

        match get_branch_for_commit(&repo, &parent_commit)? {
            Some(branch) => return Ok(branch.name()?.unwrap().to_string()),
            None => commit = parent_commit,
        };
    }

    Ok(main_branch_name)
}
