use anyhow::{Context, Result};
use cucumber::World as CucumberWorld;
use std::path::PathBuf;
use tempfile::TempDir;

#[derive(Debug, CucumberWorld)]
#[world(init = Self::new)]
pub struct World {
    pub repo_path: PathBuf,
    pub _temp_dir: TempDir,
    pub output: String,
    pub exit_code: i32,
}

impl World {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
        let repo_path = temp_dir.path().to_path_buf();

        Ok(Self {
            repo_path,
            _temp_dir: temp_dir,
            output: String::new(),
            exit_code: 0,
        })
    }
}

pub fn strip_ansi_codes(s: &str) -> String {
    // Simple ANSI code stripper - matches ESC[...m patterns
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}
