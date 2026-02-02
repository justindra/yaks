// Git ref sync adapter - stub implementation

use crate::ports::SyncPort;
use anyhow::Result;

pub struct GitRefSync;

impl GitRefSync {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl SyncPort for GitRefSync {
    fn push(&self) -> Result<()> {
        // TODO: Implement git ref push
        anyhow::bail!("Git ref sync not yet implemented")
    }

    fn pull(&self) -> Result<()> {
        // TODO: Implement git ref pull
        anyhow::bail!("Git ref sync not yet implemented")
    }

    fn sync(&self) -> Result<()> {
        // TODO: Implement git ref sync
        anyhow::bail!("Git ref sync not yet implemented")
    }
}
