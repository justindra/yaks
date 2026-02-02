// Directory-based storage adapter - implements .yaks/ directory structure

use crate::domain::Yak;
use crate::ports::StoragePort;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct DirectoryStorage {
    base_path: PathBuf,
}

impl DirectoryStorage {
    pub fn new() -> Result<Self> {
        let base_path = std::env::var("YAK_PATH")
            .unwrap_or_else(|_| ".yaks".to_string())
            .into();

        Ok(Self { base_path })
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
        Ok(())
    }

    fn get_yak(&self, name: &str) -> Result<Yak> {
        let dir = self.yak_dir(name);
        if !dir.exists() {
            anyhow::bail!("Yak '{}' does not exist", name);
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

        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
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
            anyhow::bail!("Yak '{}' does not exist", from);
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
        assert!(result.unwrap_err().to_string().contains("does not exist"));
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
