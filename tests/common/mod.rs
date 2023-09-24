use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use flate2::read::GzDecoder;
use git2::Repository;
use tar::Archive;
use tempfile::{tempdir, TempDir};

pub const TEST_BINARY: &str = "git-github-pull-request";

pub fn restore_git_repo(tar_gz: &str) -> Result<TempDir> {
    let mut repo_tar_gz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    repo_tar_gz.push("tests");
    repo_tar_gz.push("artifacts");
    repo_tar_gz.push(tar_gz);

    let t = tempdir()?;

    let tar_gz_file = File::open(repo_tar_gz)?;
    let tar_file = GzDecoder::new(tar_gz_file);
    Archive::new(tar_file).unpack(t.path())?;

    Ok(t)
}

pub fn current_branch_name(repository_path: &Path) -> Result<String> {
    let repo = Repository::discover(repository_path)?;

    let head = repo.head()?;

    if !head.is_branch() {
        return Err(anyhow!("No branch selected."));
    }

    head.symbolic_target()
        .map(|s| s.to_string())
        .ok_or(anyhow!("No branch selected."))
}
