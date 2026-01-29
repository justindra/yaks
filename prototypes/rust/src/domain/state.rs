use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YakState {
    Todo,
    Done,
}

impl YakState {
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "done" => YakState::Done,
            _ => YakState::Todo,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            YakState::Todo => "todo",
            YakState::Done => "done",
        }
    }
}

impl fmt::Display for YakState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
