use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use speculoos::prelude::*;
use tempfile::tempdir;

#[test]
fn without_valid_repository() -> Result<()> {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("git-github-pull-request")
        .current_release()
        .current_target()
        .run()?;

    let tmp_dir = tempdir()?;
    std::env::set_current_dir(&tmp_dir)?;

    let output = bin_under_test.command().arg("create").output()?;

    let stderr = String::from_utf8(output.stderr.clone())?;

    assert_that(&stderr)
        .starts_with("Could not find a repository. Has `git init` been run?");

    output.assert().failure();

    // Close explicitly so errors get reported.
    tmp_dir.close()?;

    Ok(())
}
