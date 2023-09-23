use anyhow::Result;
use speculoos::prelude::*;

use crate::common::restore_git_repo;
use crate::common::TEST_BINARY;

mod common;

/// Tests what happens when command is invoked for a repository that is
/// initialized, but contains no commits.
#[test]
fn initialized_no_commits() -> Result<()> {
    //
    // Arrange.
    //
    let t = restore_git_repo("initialized_no_commits.tar.gz")?;

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
        .current_dir(&t)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that(&String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that(&String::from_utf8(output.stderr.clone())?)
        .starts_with("No currently selected commit.");
    assert_that(&output.status.success()).is_false();

    // Close explicitly so errors get reported.
    t.close()?;

    Ok(())
}

/// Tests what happens for `create` on a commit that has one existing branch.
#[test]
fn existing_branch() -> Result<()> {
    //
    // Arrange.
    //
    let t = restore_git_repo("existing_branch.tar.gz")?;

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
        .current_dir(&t)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that(&String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that(&String::from_utf8(output.stderr.clone())?).is_empty();
    assert_that(&output.status.success()).is_true();

    // Close explicitly so errors get reported.
    t.close()?;

    Ok(())
}

/// Tests what happens for `create` on a commit that doesn't have a branch.
#[test]
fn no_branch() -> Result<()> {
    //
    // Arrange.
    //
    let t = restore_git_repo("no_branch.tar.gz")?;

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
        .current_dir(&t)
        .arg("create")
        .output()?;

    //
    // Assert.
    //
    assert_that(&String::from_utf8(output.stdout.clone())?).is_empty();
    assert_that(&String::from_utf8(output.stderr.clone())?)
        .starts_with("Could not create branch ");
    assert_that(&output.status.success()).is_false();

    // Close explicitly so errors get reported.
    t.close()?;

    Ok(())
}
