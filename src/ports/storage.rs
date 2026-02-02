// Storage port trait - abstraction for yak persistence

use crate::domain::Yak;
use anyhow::Result;

pub trait StoragePort {
    /// Create a new yak
    fn create_yak(&self, name: &str) -> Result<()>;

    /// Get a yak by name
    fn get_yak(&self, name: &str) -> Result<Yak>;

    /// List all yaks
    fn list_yaks(&self) -> Result<Vec<Yak>>;

    /// Mark a yak as done or undone
    fn mark_done(&self, name: &str, done: bool) -> Result<()>;

    /// Delete a yak
    fn delete_yak(&self, name: &str) -> Result<()>;

    /// Rename a yak
    fn rename_yak(&self, from: &str, to: &str) -> Result<()>;

    /// Read context for a yak
    fn read_context(&self, name: &str) -> Result<String>;

    /// Write context for a yak
    fn write_context(&self, name: &str, text: &str) -> Result<()>;

    /// Find a yak by name or fuzzy match
    /// Returns the exact name if found, or a unique fuzzy match
    /// Returns error if not found or ambiguous
    fn find_yak(&self, name: &str) -> Result<String>;
}
