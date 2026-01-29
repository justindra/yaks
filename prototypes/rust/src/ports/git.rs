use anyhow::Result;

pub trait GitRepository {
    fn is_repository(&self) -> bool;
    fn has_origin_remote(&self) -> bool;
    fn check_ignore(&self, path: &str) -> Result<bool>;
    fn log_command(&self, message: &str) -> Result<()>;
    fn sync(&self) -> Result<()>;
}
