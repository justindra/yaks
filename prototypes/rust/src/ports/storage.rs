use crate::domain::{Yak, YakState};
use anyhow::Result;

pub trait YakStorage {
    fn add(&mut self, name: &str) -> Result<()>;
    fn list(&self) -> Result<Vec<Yak>>;
    fn get(&self, name: &str) -> Result<Option<Yak>>;
    fn update_state(&mut self, name: &str, state: YakState) -> Result<()>;
    fn update_state_recursive(&mut self, name: &str, state: YakState) -> Result<()>;
    fn remove(&mut self, name: &str) -> Result<()>;
    fn set_context(&mut self, name: &str, context: &str) -> Result<()>;
    fn get_context(&self, name: &str) -> Result<String>;
    fn rename(&mut self, old_name: &str, new_name: &str) -> Result<()>;
    fn find_yak(&self, search_term: &str) -> Result<Option<String>>;
    fn migrate_done_to_state(&mut self) -> Result<()>;
}
