// EditContext use case - opens editor for yak context or reads from stdin

use crate::ports::{OutputPort, StoragePort};
use anyhow::{Context as AnyhowContext, Result};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::Command;

pub struct EditContext<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> EditContext<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(name)?;

        // Read current context
        let current_context = self.storage.read_context(&resolved_name).unwrap_or_default();

        // Check if stdin is a terminal
        let content = if atty::is(atty::Stream::Stdin) {
            // Interactive mode - launch editor
            self.edit_with_editor(&current_context)?
        } else {
            // Non-interactive mode - read from stdin
            self.read_from_stdin()?
        };

        // Write updated context
        self.storage.write_context(&resolved_name, &content)?;

        Ok(())
    }

    fn edit_with_editor(&self, initial_content: &str) -> Result<String> {
        // Get editor from environment or default to vi
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

        // Create a temporary file with the current context
        let temp_file = tempfile::NamedTempFile::new()
            .context("Failed to create temporary file")?;
        let temp_path = temp_file.path();

        // Write current context to temp file
        fs::write(temp_path, initial_content)
            .context("Failed to write initial content to temp file")?;

        // Launch editor
        let status = Command::new(&editor)
            .arg(temp_path)
            .status()
            .context(format!("Failed to launch editor: {}", editor))?;

        if !status.success() {
            anyhow::bail!("Editor exited with non-zero status");
        }

        // Read edited content
        let content = fs::read_to_string(temp_path)
            .context("Failed to read edited content")?;

        Ok(content)
    }

    fn read_from_stdin(&self) -> Result<String> {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Yak;
    use std::cell::RefCell;

    struct MockStorage {
        yaks: RefCell<Vec<Yak>>,
        contexts: RefCell<std::collections::HashMap<String, String>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                yaks: RefCell::new(Vec::new()),
                contexts: RefCell::new(std::collections::HashMap::new()),
            }
        }

        fn add_yak(&self, name: &str) {
            self.yaks.borrow_mut().push(Yak {
                name: name.to_string(),
                done: false,
                context: None,
            });
        }

        fn set_context(&self, name: &str, context: &str) {
            self.contexts
                .borrow_mut()
                .insert(name.to_string(), context.to_string());
        }

        fn get_context(&self, name: &str) -> Option<String> {
            self.contexts.borrow().get(name).cloned()
        }
    }

    impl StoragePort for MockStorage {
        fn create_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn get_yak(&self, name: &str) -> Result<Yak> {
            self.yaks
                .borrow()
                .iter()
                .find(|y| y.name == name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("yak '{}' not found", name))
        }

        fn list_yaks(&self) -> Result<Vec<Yak>> {
            unimplemented!()
        }

        fn mark_done(&self, _name: &str, _done: bool) -> Result<()> {
            unimplemented!()
        }

        fn delete_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn rename_yak(&self, _from: &str, _to: &str) -> Result<()> {
            unimplemented!()
        }

        fn read_context(&self, name: &str) -> Result<String> {
            Ok(self.get_context(name).unwrap_or_default())
        }

        fn write_context(&self, name: &str, text: &str) -> Result<()> {
            self.set_context(name, text);
            Ok(())
        }

        fn find_yak(&self, name: &str) -> Result<String> {
            self.get_yak(name)?;
            Ok(name.to_string())
        }
    }

    struct MockOutput;

    impl OutputPort for MockOutput {
        fn success(&self, _message: &str) {}
        fn error(&self, _message: &str) {}
        fn info(&self, _message: &str) {}
    }

    #[test]
    fn test_edit_context_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput;
        let use_case = EditContext::new(&storage, &output);

        let result = use_case.execute("nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // Note: Full editor interaction testing is done in integration tests.
    // Unit tests here focus on validation logic.
}
