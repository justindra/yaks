use anyhow::{Context, Result};
use cucumber::{given, then, when};
use std::process::Command;

use super::world::{strip_ansi_codes, World};

#[given(expr = "I have a clean git repository")]
async fn clean_git_repo(world: &mut World) -> Result<()> {
    // Initialize git repository
    let status = Command::new("git")
        .arg("init")
        .current_dir(&world.repo_path)
        .status()
        .context("Failed to run git init")?;

    if !status.success() {
        anyhow::bail!("git init failed");
    }

    // Configure git user for the test repo
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&world.repo_path)
        .status()
        .context("Failed to set git user.email")?;

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&world.repo_path)
        .status()
        .context("Failed to set git user.name")?;

    Ok(())
}

#[given(regex = r#"I have added the yak "(.+)""#)]
async fn add_yak(world: &mut World, yak_name: String) -> Result<()> {
    // Use the yx binary to add a yak
    let yx_path = env!("CARGO_BIN_EXE_yx");

    let output = Command::new(yx_path)
        .arg("add")
        .arg(&yak_name)
        .env("YAK_PATH", &world.repo_path)
        .env("YX_IGNORE_STDIN", "1") // Skip interactive editor
        .env("YX_SKIP_GIT_CHECKS", "1") // Skip git logging
        .current_dir(&world.repo_path)
        .output()
        .context("Failed to run yx add")?;

    if !output.status.success() {
        anyhow::bail!(
            "yx add failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[when(regex = r#"I run "(.+)""#)]
async fn run_command(world: &mut World, command: String) -> Result<()> {
    // Parse the command string
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }

    // For yx commands, use the binary
    if parts[0] == "yx" {
        let yx_path = env!("CARGO_BIN_EXE_yx");

        let output = Command::new(yx_path)
            .args(&parts[1..])
            .env("YAK_PATH", &world.repo_path)
            .env("YX_SKIP_GIT_CHECKS", "1") // Skip git logging
            .current_dir(&world.repo_path)
            .output()
            .context("Failed to run yx command")?;

        world.exit_code = output.status.code().unwrap_or(-1);
        world.output = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(())
    } else {
        anyhow::bail!("Unsupported command: {}", parts[0])
    }
}

#[then(expr = "the output should be:")]
async fn output_should_be(world: &mut World, step: &cucumber::gherkin::Step) -> Result<()> {
    // Get the docstring from the step
    let expected = step
        .docstring
        .as_ref()
        .context("Expected docstring in step")?;

    let expected_text = expected.trim();
    let actual = world.output.trim();

    // Strip ANSI color codes from actual output for comparison
    let actual_no_ansi = strip_ansi_codes(actual);

    if actual_no_ansi != expected_text {
        anyhow::bail!(
            "\nExpected:\n{}\n\nActual:\n{}",
            expected_text,
            actual_no_ansi
        );
    }

    Ok(())
}
