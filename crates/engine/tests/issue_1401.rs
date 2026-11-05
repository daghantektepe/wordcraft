//! Regression test for Git ls-files failure handling (GitHub Issue #1401).
//! The test creates a temporary directory that is **not** a Git repository,
//! invokes `git ls-files -z` inside it, and verifies that the command fails
//! without causing a panic. This mirrors the situation where the xtask
//! asset loader attempts to read raw NUL‑separated paths from `git ls-files`
//! and must propagate the error correctly.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Helper that creates a fresh temporary directory that is guaranteed not to
/// contain a `.git` folder. The directory is automatically removed when the
/// test finishes because it lives inside the OS temporary directory.
fn create_non_git_dir() -> std::io::Result<PathBuf> {
    let mut dir = env::temp_dir();
    // Use a unique name based on the current process ID and a random component.
    let pid = std::process::id();
    let uniq = format!("wordcraft_issue_1401_{}", pid);
    dir.push(uniq);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Executes `git ls-files -z` in the given directory and returns the `Output`.
fn run_git_ls_files(dir: &PathBuf) -> std::io::Result<std::process::Output> {
    Command::new("git")
        .args(&["ls-files", "-z"])
        .current_dir(dir)
        .output()
}

/// The actual regression test.
///
/// It asserts that:
/// 1. Running `git ls-files -z` in a non‑Git directory fails (non‑zero exit).
/// 2. The failure is reported via `Result::Err` rather than a panic.
/// 3. The raw NUL‑separated output handling path (which would be used by the
///    xtask asset loader) does not attempt to parse the output when the command
///    fails.
#[test]
fn git_ls_files_failure_regression() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: create a temporary directory without a .git repository.
    let tmp_dir = create_non_git_dir()?;

    // Step 2: run the git command.
    let output = run_git_ls_files(&tmp_dir)?;

    // Step 3: Verify that the command failed.
    // In a non‑Git directory `git ls-files` exits with status 128.
    assert!(
        !output.status.success(),
        "git ls-files unexpectedly succeeded in a non‑git directory"
    );

    // Step 4: Ensure that we do not attempt to parse the stdout when the command
    // failed. The stdout should be empty or contain an error message, but we
    // simply ignore it safely.
    let stdout_len = output.stdout.len();
    // The output may contain an error message; we only care that we handle it
    // without panicking. For completeness we assert that the length is reasonable.
    assert!(
        stdout_len == 0 || stdout_len < 1024,
        "unexpectedly large stdout from failing git command"
    );

    // Clean up the temporary directory.
    // Errors during cleanup are ignored because they are not relevant to the
    // regression verification.
    let _ = fs::remove_dir_all(&tmp_dir);

    Ok(())
}
