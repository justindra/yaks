// MoveYak use case - renames/relocates a yak

use crate::domain::validate_yak_name;
use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct MoveYak<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> MoveYak<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, from: &str, to: &str) -> Result<()> {
        // Validate new name
        validate_yak_name(to)
            .map_err(|e| anyhow::anyhow!(e))?;

        // Validate source exists
        let _yak = self.storage.get_yak(from)?;

        // Rename the yak
        self.storage.rename_yak(from, to)?;

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

        fn delete_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn rename_yak(&self, from: &str, to: &str) -> Result<()> {
            let mut yaks = self.yaks.borrow_mut();

            // Check source exists
            if !yaks.iter().any(|y| y.name == from) {
                anyhow::bail!("Yak '{}' does not exist", from);
            }

            // Check target doesn't exist
            if yaks.iter().any(|y| y.name == to) {
                anyhow::bail!("Yak '{}' already exists", to);
            }

            // Rename the yak
            if let Some(yak) = yaks.iter_mut().find(|y| y.name == from) {
                yak.name = to.to_string();
            }

            Ok(())
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
    fn test_move_yak_renames_yak() {
        let storage = MockStorage::new();
        storage.add_yak("old-name", false);
        let output = MockOutput::new();
        let use_case = MoveYak::new(&storage, &output);

        use_case.execute("old-name", "new-name").unwrap();

        assert!(!storage.yak_exists("old-name"));
        assert!(storage.yak_exists("new-name"));
    }

    #[test]
    fn test_move_yak_outputs_success() {
        let storage = MockStorage::new();
        storage.add_yak("old-name", false);
        let output = MockOutput::new();
        let use_case = MoveYak::new(&storage, &output);

        use_case.execute("old-name", "new-name").unwrap();

        assert_eq!(
            output.last_message(),
            Some("Moved 'old-name' to 'new-name'".to_string())
        );
    }

    #[test]
    fn test_move_yak_fails_for_nonexistent_source() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = MoveYak::new(&storage, &output);

        let result = use_case.execute("nonexistent", "new-name");

        assert!(result.is_err());
    }

    #[test]
    fn test_move_yak_fails_for_existing_target() {
        let storage = MockStorage::new();
        storage.add_yak("old-name", false);
        storage.add_yak("new-name", false);
        let output = MockOutput::new();
        let use_case = MoveYak::new(&storage, &output);

        let result = use_case.execute("old-name", "new-name");

        assert!(result.is_err());
    }
}
