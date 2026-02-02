// Git-based log adapter - commits yak operations to refs/notes/yaks

use crate::ports::LogPort;
use anyhow::{Context, Result};
use git2::Repository;
use std::path::PathBuf;

pub struct GitLog {
    repo: Repository,
    yaks_path: PathBuf,
}

impl GitLog {
    pub fn new() -> Result<Self> {
        let git_work_tree = std::env::var("GIT_WORK_TREE")
            .or_else(|_| std::env::current_dir().map(|p| p.display().to_string()))?;

        let repo = Repository::open(&git_work_tree)
            .with_context(|| format!("Failed to open git repository at {git_work_tree}"))?;

        let yak_path_str = std::env::var("YAK_PATH").unwrap_or_else(|_| ".yaks".to_string());

        // Resolve yaks_path relative to git_work_tree if it's relative
        let yaks_path = if std::path::Path::new(&yak_path_str).is_absolute() {
            PathBuf::from(yak_path_str)
        } else {
            PathBuf::from(&git_work_tree).join(yak_path_str)
        };

        Ok(Self { repo, yaks_path })
    }

    // Build a tree from .yaks directory
    fn build_tree_from_yaks(&self) -> Result<git2::Oid> {
        let mut index = git2::Index::new()?;

        if self.yaks_path.exists() {
            for entry in walkdir::WalkDir::new(&self.yaks_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                let relative = path.strip_prefix(&self.yaks_path)?;
                let contents = std::fs::read(path)?;

                // Create blob from file contents
                let oid = self.repo.blob(&contents)?;

                // Add to index
                let index_entry = git2::IndexEntry {
                    ctime: git2::IndexTime::new(0, 0),
                    mtime: git2::IndexTime::new(0, 0),
                    dev: 0,
                    ino: 0,
                    mode: 0o100644, // regular file
                    uid: 0,
                    gid: 0,
                    file_size: contents.len() as u32,
                    id: oid,
                    flags: 0,
                    flags_extended: 0,
                    path: relative.to_str().unwrap().as_bytes().to_vec(),
                };
                index.add(&index_entry)?;
            }
        }

        let tree_oid = index.write_tree_to(&self.repo)?;
        Ok(tree_oid)
    }

    // Get the OID of refs/notes/yaks if it exists
    fn get_local_ref(&self) -> Result<Option<git2::Oid>> {
        match self.repo.refname_to_id("refs/notes/yaks") {
            Ok(oid) => Ok(Some(oid)),
            Err(_) => Ok(None),
        }
    }
}

impl LogPort for GitLog {
    fn log_command(&self, command: &str) -> Result<()> {
        // Skip if not in a git repo or yaks path doesn't exist
        if !self.yaks_path.exists() {
            return Ok(());
        }

        let tree_oid = self.build_tree_from_yaks()?;
        let tree = self.repo.find_tree(tree_oid)?;

        // Get parent commit if refs/notes/yaks exists
        let parent = self
            .get_local_ref()?
            .and_then(|oid| self.repo.find_commit(oid).ok());

        let parents: Vec<_> = parent.iter().collect();

        // Create commit
        let sig = self.repo.signature()?;
        self.repo.commit(
            Some("refs/notes/yaks"),
            &sig,
            &sig,
            command,
            &tree,
            &parents,
        )?;

        Ok(())
    }
}
