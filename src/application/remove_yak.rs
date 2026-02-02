// RemoveYak use case - deletes a yak

use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct RemoveYak<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> RemoveYak<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        // Check if yak exists first
        let _yak = self.storage.get_yak(name)?;

        // Delete the yak
        self.storage.delete_yak(name)?;

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
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                yaks: RefCell::new(Vec::new()),
            }
        }

        fn add_yak(&self, name: &str, done: bool) {
            self.yaks.borrow_mut().push(Yak {
                name: name.to_string(),
                done,
                context: None,
            });
        }

        fn yak_exists(&self, name: &str) -> bool {
            self.yaks.borrow().iter().any(|y| y.name == name)
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

        fn delete_yak(&self, name: &str) -> Result<()> {
            let mut yaks = self.yaks.borrow_mut();
            if let Some(pos) = yaks.iter().position(|y| y.name == name) {
                yaks.remove(pos);
                Ok(())
            } else {
                anyhow::bail!("Yak '{}' does not exist", name)
            }
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
    fn test_remove_yak_deletes_yak() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", false);
        let output = MockOutput::new();
        let use_case = RemoveYak::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        assert!(!storage.yak_exists("test-yak"));
    }

    #[test]
    fn test_remove_yak_outputs_success() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", false);
        let output = MockOutput::new();
        let use_case = RemoveYak::new(&storage, &output);

        use_case.execute("test-yak").unwrap();

        assert_eq!(
            output.last_message(),
            Some("Removed 'test-yak'".to_string())
        );
    }

    #[test]
    fn test_remove_yak_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = RemoveYak::new(&storage, &output);

        let result = use_case.execute("nonexistent");

        assert!(result.is_err());
    }
}
