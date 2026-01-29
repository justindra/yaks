use super::state::YakState;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Yak {
    pub name: String,
    pub state: YakState,
    pub context: String,
    pub mtime: SystemTime,
}

impl Yak {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: YakState::Todo,
            context: String::new(),
            mtime: SystemTime::now(),
        }
    }

    pub fn with_state(mut self, state: YakState) -> Self {
        self.state = state;
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }

    pub fn with_mtime(mut self, mtime: SystemTime) -> Self {
        self.mtime = mtime;
        self
    }

    pub fn basename(&self) -> &str {
        self.name.rsplit('/').next().unwrap_or(&self.name)
    }

    pub fn depth(&self) -> usize {
        self.name.matches('/').count()
    }

    pub fn parent(&self) -> Option<String> {
        if let Some(pos) = self.name.rfind('/') {
            Some(self.name[..pos].to_string())
        } else {
            None
        }
    }

    pub fn is_child_of(&self, parent: &str) -> bool {
        if parent.is_empty() || parent == "." {
            return !self.name.contains('/');
        }
        self.name.starts_with(&format!("{}/", parent))
            && self.name[parent.len() + 1..].matches('/').count() == 0
    }
}
