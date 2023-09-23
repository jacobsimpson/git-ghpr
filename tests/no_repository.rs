use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use speculoos::prelude::*;
use tempdir::TempDir;

#[test]
fn without_valid_repository() -> Result<()> {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("git-github-pull-request")
        .current_release()
        .current_target()
        .run()?;

    let tmp_dir = TempDir::new("prtest")?;

    std::env::set_current_dir(&tmp_dir)?;
    let output = bin_under_test.command().arg("create").output()?;

    let stderr = String::from_utf8(output.stderr.clone())?;

    // Error: could not find repository from '.'; class=Repository (6); code=NotFound (-3)
    assert_that(&stderr)
        .starts_with("Error: could not find repository from '.';");

    output.assert().failure();

    // Close explicitly so errors get captured.
    tmp_dir.close()?;

    Ok(())
}