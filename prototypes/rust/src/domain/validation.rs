use anyhow::{bail, Result};
use regex::Regex;
use std::sync::OnceLock;

static FORBIDDEN_CHARS: OnceLock<Regex> = OnceLock::new();

fn get_forbidden_chars_regex() -> &'static Regex {
    FORBIDDEN_CHARS.get_or_init(|| {
        Regex::new(r#"[\\:*?|<>"]"#).unwrap()
    })
}

pub fn validate_yak_name(name: &str) -> Result<()> {
    let regex = get_forbidden_chars_regex();
    if regex.is_match(name) {
        bail!("Invalid yak name: contains forbidden characters (\\ : * ? | < > \")");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(validate_yak_name("simple-name").is_ok());
        assert!(validate_yak_name("parent/child").is_ok());
        assert!(validate_yak_name("with spaces").is_ok());
    }

    #[test]
    fn test_invalid_names() {
        assert!(validate_yak_name("with\\backslash").is_err());
        assert!(validate_yak_name("with:colon").is_err());
        assert!(validate_yak_name("with*asterisk").is_err());
        assert!(validate_yak_name("with?question").is_err());
        assert!(validate_yak_name("with|pipe").is_err());
        assert!(validate_yak_name("with<less").is_err());
        assert!(validate_yak_name("with>greater").is_err());
        assert!(validate_yak_name("with\"quote").is_err());
    }
}
