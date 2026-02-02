// Directory-based storage adapter - implements .yaks/ directory structure

use crate::domain::Yak;
use crate::ports::StoragePort;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

pub struct DirectoryStorage {
    base_path: PathBuf,
}

impl DirectoryStorage {
    pub fn new() -> Result<Self> {
        // Check 1: Is git command available?
        Self::check_git_available()?;

        // Check 2: Are we in a git repository?
        Self::check_in_git_repo()?;

        // Check 3: Is .yaks gitignored?
        Self::check_yaks_gitignored()?;

        // Priority: YAK_PATH env var, then GIT_WORK_TREE/.yaks, then .yaks
        // This matches bash version behavior: YAKS_PATH="$GIT_WORK_TREE/.yaks"
        let base_path = if let Ok(yak_path) = std::env::var("YAK_PATH") {
            yak_path.into()
        } else if let Ok(git_work_tree) = std::env::var("GIT_WORK_TREE") {
            PathBuf::from(git_work_tree).join(".yaks")
        } else {
            ".yaks".into()
        };

        Ok(Self { base_path })
    }

    fn check_git_available() -> Result<()> {
        // Try to run "git --version" to check if git command exists
        let output = Command::new("git")
            .arg("--version")
            .output();

        match output {
            Ok(_) => Ok(()),
            Err(_) => anyhow::bail!("Error: git command not found"),
        }
    }

    fn check_in_git_repo() -> Result<()> {
        // Run "git rev-parse --git-dir" to check if we're in a git repository
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--git-dir")
            .output()
            .context("Failed to check git repository")?;

        if !output.status.success() {
            anyhow::bail!("Error: not in a git repository");
        }

        Ok(())
    }

    fn check_yaks_gitignored() -> Result<()> {
        // Run "git check-ignore .yaks" to verify .yaks is gitignored
        let output = Command::new("git")
            .arg("check-ignore")
            .arg(".yaks")
            .output()
            .context("Failed to check .yaks gitignore status")?;

        // git check-ignore returns exit code 0 if the path is ignored
        if !output.status.success() {
            anyhow::bail!("Error: .yaks folder is not gitignored");
        }

        Ok(())
    }

    fn yak_dir(&self, name: &str) -> PathBuf {
        self.base_path.join(name)
    }

    fn done_marker_path(&self, name: &str) -> PathBuf {
        self.yak_dir(name).join("done")
    }

    fn context_path(&self, name: &str) -> PathBuf {
        self.yak_dir(name).join("context.md")
    }
}

impl StoragePort for DirectoryStorage {
    fn create_yak(&self, name: &str) -> Result<()> {
        let dir = self.yak_dir(name);
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create yak directory: {}", name))?;

        // Create empty context.md file by default
        let context_file = self.context_path(name);
        fs::write(&context_file, "")
            .with_context(|| format!("Failed to create context.md for yak: {}", name))?;

        Ok(())
    }

    fn get_yak(&self, name: &str) -> Result<Yak> {
        let dir = self.yak_dir(name);
        if !dir.exists() {
            anyhow::bail!("yak '{}' not found", name);
        }

        let done = self.done_marker_path(name).exists();
        let context = self.read_context(name).ok();

        Ok(Yak {
            name: name.to_string(),
            done,
            context,
        })
    }

    fn list_yaks(&self) -> Result<Vec<Yak>> {
        let mut yaks = Vec::new();

        if !self.base_path.exists() {
            return Ok(yaks);
        }

        // Use WalkDir to recursively find all directories (yaks)
        for entry in WalkDir::new(&self.base_path)
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| e.file_type().is_dir())
        {
            let entry = entry?;
            // Get relative path from base_path
            if let Ok(rel_path) = entry.path().strip_prefix(&self.base_path) {
                if let Some(name) = rel_path.to_str() {
                    // Only add if we can successfully read it as a yak
                    if let Ok(yak) = self.get_yak(name) {
                        yaks.push(yak);
                    }
                }
            }
        }

        Ok(yaks)
    }

    fn mark_done(&self, name: &str, done: bool) -> Result<()> {
        let marker = self.done_marker_path(name);

        if done {
            fs::write(&marker, "")
                .with_context(|| format!("Failed to mark '{}' as done", name))?;
        } else if marker.exists() {
            fs::remove_file(&marker)
                .with_context(|| format!("Failed to mark '{}' as undone", name))?;
        }

        Ok(())
    }

    fn delete_yak(&self, name: &str) -> Result<()> {
        let dir = self.yak_dir(name);
        if dir.exists() {
            fs::remove_dir_all(&dir)
                .with_context(|| format!("Failed to remove yak '{}'", name))?;
        }
        Ok(())
    }

    fn rename_yak(&self, from: &str, to: &str) -> Result<()> {
        let from_dir = self.yak_dir(from);
        let to_dir = self.yak_dir(to);

        if !from_dir.exists() {
            anyhow::bail!("yak '{}' not found", from);
        }

        if to_dir.exists() {
            anyhow::bail!("Yak '{}' already exists", to);
        }

        fs::rename(&from_dir, &to_dir)
            .with_context(|| format!("Failed to rename '{}' to '{}'", from, to))?;

        Ok(())
    }

    fn read_context(&self, name: &str) -> Result<String> {
        let path = self.context_path(name);
        fs::read_to_string(&path)
            .with_context(|| format!("Failed to read context for '{}'", name))
    }

    fn write_context(&self, name: &str, text: &str) -> Result<()> {
        let path = self.context_path(name);
        fs::write(&path, text)
            .with_context(|| format!("Failed to write context for '{}'", name))
    }

    fn find_yak(&self, name: &str) -> Result<String> {
        // First, try exact match
        if self.yak_dir(name).exists() {
            return Ok(name.to_string());
        }

        // If not found, try fuzzy match
        let yaks = self.list_yaks()?;
        let matches: Vec<&Yak> = yaks
            .iter()
            .filter(|yak| yak.name.contains(name))
            .collect();

        match matches.len() {
            0 => anyhow::bail!("yak '{}' not found", name),
            1 => Ok(matches[0].name.clone()),
            _ => anyhow::bail!("yak name '{}' is ambiguous", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_storage() -> (DirectoryStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("YAK_PATH", temp_dir.path().to_str().unwrap());
        let storage = DirectoryStorage::new().unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_create_yak() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("test-yak").unwrap();
        assert!(storage.yak_dir("test-yak").exists());
    }

    #[test]
    fn test_get_yak() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("test-yak").unwrap();
        let yak = storage.get_yak("test-yak").unwrap();
        assert_eq!(yak.name, "test-yak");
        assert!(!yak.done);
    }

    #[test]
    fn test_list_yaks() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("yak1").unwrap();
        storage.create_yak("yak2").unwrap();
        let yaks = storage.list_yaks().unwrap();
        assert_eq!(yaks.len(), 2);
    }

    #[test]
    fn test_mark_done() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("test-yak").unwrap();
        storage.mark_done("test-yak", true).unwrap();
        let yak = storage.get_yak("test-yak").unwrap();
        assert!(yak.done);
    }

    #[test]
    fn test_delete_yak() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("test-yak").unwrap();
        storage.delete_yak("test-yak").unwrap();
        assert!(!storage.yak_dir("test-yak").exists());
    }

    #[test]
    fn test_context() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("test-yak").unwrap();
        storage.write_context("test-yak", "Test context").unwrap();
        let context = storage.read_context("test-yak").unwrap();
        assert_eq!(context, "Test context");
    }

    #[test]
    fn test_rename_yak() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("old-name").unwrap();
        storage.write_context("old-name", "Context text").unwrap();
        storage.mark_done("old-name", true).unwrap();

        storage.rename_yak("old-name", "new-name").unwrap();

        assert!(!storage.yak_dir("old-name").exists());
        assert!(storage.yak_dir("new-name").exists());

        let yak = storage.get_yak("new-name").unwrap();
        assert_eq!(yak.name, "new-name");
        assert!(yak.done);
        assert_eq!(yak.context.unwrap(), "Context text");
    }

    #[test]
    fn test_rename_nonexistent_yak() {
        let (storage, _temp) = setup_test_storage();
        let result = storage.rename_yak("nonexistent", "new-name");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_rename_to_existing_yak() {
        let (storage, _temp) = setup_test_storage();
        storage.create_yak("yak1").unwrap();
        storage.create_yak("yak2").unwrap();
        let result = storage.rename_yak("yak1", "yak2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
