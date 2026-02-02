// Sync port trait - abstraction for git ref synchronization

use anyhow::Result;

pub trait SyncPort {
    /// Push local yaks to git refs
    #[allow(dead_code)]
    fn push(&self) -> Result<()>;

    /// Pull yaks from git refs
    #[allow(dead_code)]
    fn pull(&self) -> Result<()>;

    /// Sync yaks (push + pull with merge)
    fn sync(&self) -> Result<()>;
}
