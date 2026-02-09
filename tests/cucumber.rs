use cucumber::{given, then, when, World};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Default, World)]
struct YaksWorld {
    repo_path: PathBuf,
    output: String,
    exit_code: i32,
}

#[given(expr = "I have a clean git repository")]
async fn clean_git_repo(world: &mut YaksWorld) {
    // Create temp directory
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    world.repo_path = temp_dir.path().to_path_buf();
    // Keep the directory so it doesn't get deleted
    std::mem::forget(temp_dir);

    // Initialize git repository
    let status = Command::new("git")
        .arg("init")
        .current_dir(&world.repo_path)
        .status()
        .expect("Failed to run git init");

    assert!(status.success(), "git init failed");

    // Configure git user for the test repo
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&world.repo_path)
        .status()
        .expect("Failed to set git user.email");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&world.repo_path)
        .status()
        .expect("Failed to set git user.name");
}

#[given(regex = r#"I have added the yak "(.+)""#)]
async fn add_yak(world: &mut YaksWorld, yak_name: String) {
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
        .expect("Failed to run yx add");

    if !output.status.success() {
        eprintln!("yx add failed:");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("yx add failed");
    }
}

#[when(regex = r#"I run "(.+)""#)]
async fn run_command(world: &mut YaksWorld, command: String) {
    // Parse the command string
    let parts: Vec<&str> = command.split_whitespace().collect();
    assert!(!parts.is_empty(), "Empty command");

    // For yx commands, use the binary
    if parts[0] == "yx" {
        let yx_path = env!("CARGO_BIN_EXE_yx");

        let output = Command::new(yx_path)
            .args(&parts[1..])
            .env("YAK_PATH", &world.repo_path)
            .env("YX_SKIP_GIT_CHECKS", "1") // Skip git logging
            .current_dir(&world.repo_path)
            .output()
            .expect("Failed to run yx command");

        world.exit_code = output.status.code().unwrap_or(-1);
        world.output = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        panic!("Unsupported command: {}", parts[0]);
    }
}

#[then(expr = "the output should be:")]
async fn output_should_be(world: &mut YaksWorld, step: &cucumber::gherkin::Step) {
    // Get the docstring from the step
    let expected = step.docstring.as_ref().expect("Expected docstring in step");

    let expected_text = expected.trim();
    let actual = world.output.trim();

    // Strip ANSI color codes from actual output for comparison
    let actual_no_ansi = strip_ansi_codes(actual);

    assert_eq!(
        actual_no_ansi, expected_text,
        "\nExpected:\n{}\n\nActual:\n{}",
        expected_text, actual_no_ansi
    );
}

fn strip_ansi_codes(s: &str) -> String {
    // Simple ANSI code stripper - matches ESC[...m patterns
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

#[tokio::test]
async fn run_cucumber_tests() {
    YaksWorld::run("features/list.feature").await;
}
