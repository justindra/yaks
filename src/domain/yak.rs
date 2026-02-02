// Yak domain model

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Yak {
    pub name: String,
    pub done: bool,
    pub context: Option<String>,
}

impl Yak {
    pub fn new(name: String) -> Self {
        Self {
            name,
            done: false,
            context: None,
        }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn mark_done(mut self) -> Self {
        self.done = true;
        self
    }

    pub fn mark_undone(mut self) -> Self {
        self.done = false;
        self
    }
}

/// Validate a yak name
pub fn validate_yak_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Yak name cannot be empty".to_string());
    }

    // Check for invalid characters that would cause filesystem issues
    if name.contains('\0') || name.contains('/') && cfg!(unix) {
        return Err(format!("Invalid yak name: '{}'", name));
    }

    Ok(())
}

/// Parse hierarchy from yak name (e.g., "dx/rust" -> ["dx", "rust"])
pub fn parse_hierarchy(name: &str) -> Vec<&str> {
    name.split('/').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_yak() {
        let yak = Yak::new("test".to_string());
        assert_eq!(yak.name, "test");
        assert!(!yak.done);
        assert_eq!(yak.context, None);
    }

    #[test]
    fn test_yak_with_context() {
        let yak = Yak::new("test".to_string()).with_context("Some context".to_string());
        assert_eq!(yak.context, Some("Some context".to_string()));
    }

    #[test]
    fn test_mark_done() {
        let yak = Yak::new("test".to_string()).mark_done();
        assert!(yak.done);
    }

    #[test]
    fn test_mark_undone() {
        let yak = Yak::new("test".to_string()).mark_done().mark_undone();
        assert!(!yak.done);
    }

    #[test]
    fn test_validate_yak_name_valid() {
        assert!(validate_yak_name("test").is_ok());
        assert!(validate_yak_name("dx/rust").is_ok());
    }

    #[test]
    fn test_validate_yak_name_empty() {
        assert!(validate_yak_name("").is_err());
    }

    #[test]
    fn test_validate_yak_name_null_char() {
        assert!(validate_yak_name("test\0name").is_err());
    }

    #[test]
    fn test_parse_hierarchy() {
        assert_eq!(parse_hierarchy("dx/rust"), vec!["dx", "rust"]);
        assert_eq!(parse_hierarchy("simple"), vec!["simple"]);
        assert_eq!(parse_hierarchy("a/b/c"), vec!["a", "b", "c"]);
    }
}
