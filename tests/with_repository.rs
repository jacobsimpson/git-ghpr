use anyhow::Result;
use speculoos::prelude::*;

use crate::common::current_branch_name;
use crate::common::restore_git_repo;
use crate::common::TEST_BINARY;

mod common;

/// Gets the name of the `.tar.gz` file to use for restoring the Git repository
/// for the test. It creates the `.tar.gz` file name from the name of the
/// function containing the invocation, in the format `<function_name>.tar.gz`.
macro_rules! tar_gz {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        let name = match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        };
        format!("{name}.tar.gz")
    }};
}

/// Tests what happens when command is invoked for a repository that is
/// initialized, but contains no commits.
#[test]
fn initialized_no_commits() -> Result<()> {
    //
    // Arrange.
    //
    let (_temp_dir, local_repo) =
        restore_git_repo("initialized_no_commits.tar.gz")?;

    let bin_under_test = escargot::CargoBuild::new()
        .bin(TEST_BINARY)
        .current_release()
        .current_target()
        .run()?;

    //
    // Act.
    //
    let output = bin_under_test
        .command()
        .current_dir(&local_repo)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that!(String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that!(String::from_utf8(output.stderr.clone())?)
        .starts_with("This repository has no remote.");
    assert_that!(output.status.success()).is_false();

    Ok(())
}

/// Tests what happens for `create` on a commit that has one existing branch.
#[test]
fn existing_branch() -> Result<()> {
    //
    // Arrange.
    //
    let (_temp_dir, local_repo) = restore_git_repo("existing_branch.tar.gz")?;

    let bin_under_test = escargot::CargoBuild::new()
        .bin(TEST_BINARY)
        .current_release()
        .current_target()
        .run()?;

    //
    // Act.
    //
    let output = bin_under_test
        .command()
        .current_dir(&local_repo)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that!(String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that!(String::from_utf8(output.stderr.clone())?)
        .is_equal_to("This repository has no remote.\n".to_string());
    assert_that!(output.status.success()).is_false();

    Ok(())
}

/// Tests what happens for `create` on a commit that doesn't have a branch.
///
/// ◇ 24cf4e2 22d (main) Initial commit.
/// ┃
/// ● e9f4920 22d Commit 2.
///
/// This should result in the creation of a new branch with a name based on the
/// configured branch name template.
#[test]
fn no_branch() -> Result<()> {
    //
    // Arrange.
    //
    let (_temp_dir, local_repo) = restore_git_repo("no_branch.tar.gz")?;

    let bin_under_test = escargot::CargoBuild::new()
        .bin(TEST_BINARY)
        .current_release()
        .current_target()
        .run()?;

    //
    // Act.
    //
    let output = bin_under_test
        .command()
        .current_dir(&local_repo)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that!(String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that!(String::from_utf8(output.stderr.clone())?).is_empty();
    assert_that!(output.status.success()).is_true();
    assert_that!(current_branch_name(local_repo.as_path()))
        .is_ok()
        .is_equal_to("refs/heads/commit-2".to_string());

    Ok(())
}

/// Checks what happens if there is some oddball name for the main branch of the
/// repository.
#[test]
fn unknown_main_branch() -> Result<()> {
    //
    // Arrange.
    //
    let (_temp_dir, local_repo) = restore_git_repo(&tar_gz!())?;

    let bin_under_test = escargot::CargoBuild::new()
        .bin(TEST_BINARY)
        .current_release()
        .current_target()
        .run()?;

    //
    // Act.
    //
    let output = bin_under_test
        .command()
        .current_dir(&local_repo)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that!(String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that!(String::from_utf8(output.stderr.clone())?).is_equal_to(
        "Could not find a 'main' branch. Tried 'main' and 'master'.\n"
            .to_string(),
    );
    assert_that!(output.status.success()).is_false();
    assert_that!(current_branch_name(local_repo.as_path()))
        .is_ok()
        .is_equal_to("refs/heads/commit-2".to_string());

    Ok(())
}
