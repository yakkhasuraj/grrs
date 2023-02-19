use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Used for creating temporary files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

/// It creates a temporary file, writes some content to it, runs `grrs test <file>`, and asserts that
/// the output contains the expected content
///
/// Returns:
///
/// A Result<(), Box<dyn std::error::Error>>
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

/// It runs the `grrs` binary with the arguments `foobar` and `test/file/doesnt/exist`, and asserts that
/// the command fails with an error message containing the string `Could not read file`
///
/// Returns:
///
/// A Result<(), Box<dyn std::error::Error>>
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}
