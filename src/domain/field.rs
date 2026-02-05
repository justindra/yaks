// Field domain logic - validation and reserved field names

use anyhow::Result;

/// Reserved field names that have special meaning
pub const STATE_FIELD: &str = "state";
pub const CONTEXT_FIELD: &str = "context.md";

/// All reserved field names
pub const RESERVED_FIELDS: &[&str] = &[STATE_FIELD, CONTEXT_FIELD];

/// Validate a field name for safety and reserved names
///
/// Field names must:
/// - Not be empty
/// - Only contain alphanumeric characters, hyphens, underscores, and dots
/// - Not be a reserved name (state, context.md)
/// - Not contain slashes (would create subdirectories)
pub fn validate_field_name(field_name: &str) -> Result<()> {
    // Check for empty
    if field_name.is_empty() {
        anyhow::bail!("Field name cannot be empty");
    }

    // Check for reserved names
    if RESERVED_FIELDS.contains(&field_name) {
        anyhow::bail!("Field name '{field_name}' is reserved");
    }

    // Check for valid characters (alphanumeric, hyphens, underscores, dots)
    // No slashes allowed (would create subdirectories)
    if !field_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        anyhow::bail!("Invalid field name '{field_name}' - only letters, numbers, hyphens, underscores, and dots are allowed");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_field_name_valid() {
        assert!(validate_field_name("notes").is_ok());
        assert!(validate_field_name("priority").is_ok());
        assert!(validate_field_name("notes.txt").is_ok());
        assert!(validate_field_name("my-field").is_ok());
        assert!(validate_field_name("my_field").is_ok());
    }

    #[test]
    fn test_validate_field_name_empty() {
        let result = validate_field_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_field_name_reserved_state() {
        let result = validate_field_name("state");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("reserved"));
    }

    #[test]
    fn test_validate_field_name_reserved_context() {
        let result = validate_field_name("context.md");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("reserved"));
    }

    #[test]
    fn test_validate_field_name_invalid_slash() {
        let result = validate_field_name("invalid/name");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid field name"));
    }

    #[test]
    fn test_validate_field_name_invalid_special_chars() {
        assert!(validate_field_name("field:name").is_err());
        assert!(validate_field_name("field*name").is_err());
        assert!(validate_field_name("field?name").is_err());
    }
}
