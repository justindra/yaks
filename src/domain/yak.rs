// Yak domain model

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Yak {
    pub name: String,
    pub done: bool,
    pub context: Option<String>,
}

impl Yak {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            done: false,
            context: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    #[allow(dead_code)]
    pub fn mark_done(mut self) -> Self {
        self.done = true;
        self
    }

    #[allow(dead_code)]
    pub fn mark_undone(mut self) -> Self {
        self.done = false;
        self
    }
}

/// Validate a yak name
/// Rejects names containing forbidden characters: \ : * ? | < > "
/// Slashes (/) are allowed for hierarchical yaks (e.g., "dx/rust")
pub fn validate_yak_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Yak name cannot be empty".to_string());
    }

    // Check for forbidden characters (matches bash version)
    // Forbidden: \ : * ? | < > "
    // Allowed: / (for hierarchy)
    const FORBIDDEN_CHARS: &[char] = &['\\', ':', '*', '?', '|', '<', '>', '"'];

    for c in FORBIDDEN_CHARS {
        if name.contains(*c) {
            return Err("Invalid yak name: contains forbidden characters (\\ : * ? | < > \")".to_string());
        }
    }

    Ok(())
}

/// Parse hierarchy from yak name (e.g., "dx/rust" -> ["dx", "rust"])
#[allow(dead_code)]
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
    fn test_validate_yak_name_forbidden_chars() {
        // Test each forbidden character
        assert!(validate_yak_name("test\\name").is_err());
        assert!(validate_yak_name("test:name").is_err());
        assert!(validate_yak_name("test*name").is_err());
        assert!(validate_yak_name("test?name").is_err());
        assert!(validate_yak_name("test|name").is_err());
        assert!(validate_yak_name("test<name").is_err());
        assert!(validate_yak_name("test>name").is_err());
        assert!(validate_yak_name("test\"name").is_err());

        // Slash should be allowed (for hierarchy)
        assert!(validate_yak_name("test/name").is_ok());
    }

    #[test]
    fn test_parse_hierarchy() {
        assert_eq!(parse_hierarchy("dx/rust"), vec!["dx", "rust"]);
        assert_eq!(parse_hierarchy("simple"), vec!["simple"]);
        assert_eq!(parse_hierarchy("a/b/c"), vec!["a", "b", "c"]);
    }
}
