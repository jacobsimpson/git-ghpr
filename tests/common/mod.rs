use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use flate2::read::GzDecoder;
use git2::Repository;
use tar::Archive;
use tempfile::{tempdir, TempDir};

pub const TEST_BINARY: &str = env!("CARGO_PKG_NAME");

pub fn restore_git_repo(tar_gz: &str) -> Result<TempDir> {
    let mut repo_tar_gz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    repo_tar_gz.push("tests");
    repo_tar_gz.push("resources");
    repo_tar_gz.push(tar_gz);

    let t = tempdir()?;

    let tar_gz_file = File::open(repo_tar_gz)?;
    let tar_file = GzDecoder::new(tar_gz_file);
    Archive::new(tar_file).unpack(t.path())?;

    Ok(t)
}

pub fn current_branch_name(repository_path: &Path) -> Result<String> {
    let repo = Repository::open(repository_path)?;

    for branch in repo.branches(None)? {
        match branch {
            Ok((branch, _branch_type)) => {
                println!("Found branch = {:?}", branch.name())
            }
            Err(e) => println!("Couldn't {:?}", e),
        }
    }

    let head = repo.head()?;
    // This print statement is left in here so if tests fail, this information
    // shows up in the stdout of the test results, additional context.
    println!("current_branch_name: head = {:?}", head.name());

    // For reasons I don't understand, `head.is_branch()` does not return true
    // when a branch is selected even though `git branch` in the test repository
    // confirms the branch is selected.
    let name = match head.name() {
        Some(n) => n,
        None => return Err(anyhow!("Selected branch name isn't valid UTF-8.")),
    };

    // There was a previous implmentation of this function done as follows:
    //     head.symbolic_target()
    //         .map(|s| s.to_string())
    //         .ok_or(anyhow!("No branch selected."))
    // However, when `repo.set_head(&format!("refs/heads/{branch_name}"))` is
    // used (specifically with the path information `refs/heads/{branch_name}`)
    // then `symbolic_target()` doesn't seem to behave as expected.
    if name.starts_with("refs/heads/") {
        Ok(name.to_string())
    } else {
        Err(anyhow!("No branch selected."))
    }
}
