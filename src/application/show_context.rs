// ShowContext use case - displays yak context to stdout

use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct ShowContext<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> ShowContext<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        // Validate yak exists
        let _yak = self.storage.get_yak(name)?;

        // Read context
        let context = self.storage.read_context(name).unwrap_or_default();

        // Display the header (yak name)
        self.output.info(name);

        // Display a blank line if there's content
        if !context.is_empty() {
            self.output.info("");
            // Display the context
            self.output.info(&context);
        }

        Ok(())
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
                .ok_or_else(|| anyhow::anyhow!("Yak '{}' does not exist", name))
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

        fn write_context(&self, _name: &str, _text: &str) -> Result<()> {
            unimplemented!()
        }
    }

    struct MockOutput {
        messages: RefCell<Vec<String>>,
    }

    impl MockOutput {
        fn new() -> Self {
            Self {
                messages: RefCell::new(Vec::new()),
            }
        }

        fn get_messages(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    impl OutputPort for MockOutput {
        fn success(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }

        fn error(&self, message: &str) {
            self.messages
                .borrow_mut()
                .push(format!("ERROR: {}", message));
        }

        fn info(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_show_context_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = ShowContext::new(&storage, &output);

        let result = use_case.execute("nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_show_context_displays_yak_name() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak("test-yak");
        let use_case = ShowContext::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        let messages = output.get_messages();
        assert_eq!(messages[0], "test-yak");
    }

    #[test]
    fn test_show_context_displays_empty_context() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak("test-yak");
        let use_case = ShowContext::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        let messages = output.get_messages();
        // Only the name should be displayed for empty context
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "test-yak");
    }

    #[test]
    fn test_show_context_displays_context_with_blank_line() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak("test-yak");
        storage.set_context("test-yak", "This is some context");
        let use_case = ShowContext::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0], "test-yak");
        assert_eq!(messages[1], "");
        assert_eq!(messages[2], "This is some context");
    }

    #[test]
    fn test_show_context_displays_multiline_context() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak("test-yak");
        storage.set_context("test-yak", "Line 1\nLine 2\nLine 3");
        let use_case = ShowContext::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0], "test-yak");
        assert_eq!(messages[1], "");
        assert_eq!(messages[2], "Line 1\nLine 2\nLine 3");
    }
}
