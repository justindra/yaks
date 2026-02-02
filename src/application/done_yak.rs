// DoneYak use case - marks a yak as done or undone

use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct DoneYak<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> DoneYak<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self, name: &str, undo: bool, recursive: bool) -> Result<()> {
        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(name)?;

        // If marking as done (not undo) and not recursive, check for incomplete children
        if !undo && !recursive {
            let all_yaks = self.storage.list_yaks()?;
            let has_incomplete_children = all_yaks.iter().any(|yak| {
                yak.name.starts_with(&format!("{}/", resolved_name)) && !yak.done
            });

            if has_incomplete_children {
                anyhow::bail!(
                    "cannot mark '{}' as done - it has incomplete children",
                    resolved_name
                );
            }
        }

        // If recursive, mark all children as done too
        if recursive && !undo {
            let all_yaks = self.storage.list_yaks()?;
            let children: Vec<String> = all_yaks
                .iter()
                .filter(|yak| {
                    yak.name == resolved_name || yak.name.starts_with(&format!("{}/", resolved_name))
                })
                .map(|yak| yak.name.clone())
                .collect();

            for child_name in children {
                self.storage.mark_done(&child_name, true)?;
            }
        } else {
            // Mark as done (or undone if undo flag is set)
            self.storage.mark_done(&resolved_name, !undo)?;
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

        fn get_yak_status(&self, name: &str) -> Option<bool> {
            self.yaks
                .borrow()
                .iter()
                .find(|y| y.name == name)
                .map(|y| y.done)
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

        fn mark_done(&self, name: &str, done: bool) -> Result<()> {
            let mut yaks = self.yaks.borrow_mut();
            if let Some(yak) = yaks.iter_mut().find(|y| y.name == name) {
                yak.done = done;
                Ok(())
            } else {
                anyhow::bail!("yak '{}' not found", name)
            }
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

        fn find_yak(&self, name: &str) -> Result<String> {
            // For tests, just return the name if it exists
            self.get_yak(name)?;
            Ok(name.to_string())
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
    fn test_done_yak_marks_as_done() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", false);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output);

        use_case.execute("test-yak", false).unwrap();

        assert_eq!(storage.get_yak_status("test-yak"), Some(true));
    }

    #[test]
    fn test_done_yak_outputs_success() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", false);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output);

        use_case.execute("test-yak", false).unwrap();

        assert_eq!(
            output.last_message(),
            Some("Marked 'test-yak' as done".to_string())
        );
    }

    #[test]
    fn test_done_yak_with_undo_marks_as_not_done() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", true);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output);

        use_case.execute("test-yak", true).unwrap();

        assert_eq!(storage.get_yak_status("test-yak"), Some(false));
    }

    #[test]
    fn test_done_yak_with_undo_outputs_success() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", true);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output);

        use_case.execute("test-yak", true).unwrap();

        assert_eq!(
            output.last_message(),
            Some("Marked 'test-yak' as not done".to_string())
        );
    }

    #[test]
    fn test_done_yak_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output);

        let result = use_case.execute("nonexistent", false);

        assert!(result.is_err());
    }
}
