// AddYak use case - creates a new yak

use crate::domain::validate_yak_name;
use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct AddYak<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> AddYak<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        // Validate yak name
        validate_yak_name(name)
            .map_err(|e| anyhow::anyhow!(e))?;

        self.storage.create_yak(name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Yak;
    use std::cell::RefCell;

    struct MockStorage {
        created: RefCell<Vec<String>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                created: RefCell::new(Vec::new()),
            }
        }

        fn was_created(&self, name: &str) -> bool {
            self.created.borrow().contains(&name.to_string())
        }
    }

    impl StoragePort for MockStorage {
        fn create_yak(&self, name: &str) -> Result<()> {
            self.created.borrow_mut().push(name.to_string());
            Ok(())
        }

        fn get_yak(&self, _name: &str) -> Result<Yak> {
            unimplemented!()
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

        fn read_context(&self, _name: &str) -> Result<String> {
            unimplemented!()
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

        fn last_message(&self) -> Option<String> {
            self.messages.borrow().last().cloned()
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
            self.messages
                .borrow_mut()
                .push(format!("INFO: {}", message));
        }
    }

    #[test]
    fn test_add_yak_creates_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = AddYak::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        assert!(storage.was_created("test-yak"));
    }

    #[test]
    fn test_add_yak_outputs_success() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = AddYak::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        assert_eq!(output.last_message(), Some("Added yak 'test-yak'".to_string()));
    }
}
