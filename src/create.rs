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

pub async fn create_pull_request(
    branch_name_template: &str,
    branch_name_parameters: &HashMap<String, String>,
) -> Result<Message> {
    info!("Opening the local git repository.");
    let repo = Repository::discover(".")?;

    let current_commit = get_selected_commit(&repo)?;
    info!("Current commit = {}", current_commit.id());

    let _current_branch = match get_branch_for_commit(&repo, &current_commit)? {
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
    repo.branch(&branch_name, commit, false).map_err(|e| {
        println!("{:?}", e);
        Error::UnableToCreateBranch {
            branch_name,
            base_commit: commit.id().to_string(),
        }
    })
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
