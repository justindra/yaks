// DoneYak use case - marks a yak as done or undone

use crate::domain::STATE_FIELD;
use crate::ports::{LogPort, OutputPort, StoragePort};
use anyhow::Result;

pub struct DoneYak<'a> {
    storage: &'a dyn StoragePort,
    log: &'a dyn LogPort,
}

impl<'a> DoneYak<'a> {
    pub fn new(
        storage: &'a dyn StoragePort,
        _output: &'a dyn OutputPort,
        log: &'a dyn LogPort,
    ) -> Self {
        Self { storage, log }
    }

    pub fn execute(&self, name: &str, undo: bool, recursive: bool) -> Result<()> {
        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(name)?;

        // If marking as done (not undo) and not recursive, check for incomplete children
        if !undo && !recursive {
            let all_yaks = self.storage.list_yaks()?;
            let has_incomplete_children = all_yaks
                .iter()
                .any(|yak| yak.name.starts_with(&format!("{resolved_name}/")) && !yak.done);

            if has_incomplete_children {
                anyhow::bail!("cannot mark '{resolved_name}' as done - it has incomplete children");
            }
        }

        // If recursive, mark all children as done too
        if recursive && !undo {
            let all_yaks = self.storage.list_yaks()?;
            let children: Vec<String> = all_yaks
                .iter()
                .filter(|yak| {
                    yak.name == resolved_name || yak.name.starts_with(&format!("{resolved_name}/"))
                })
                .map(|yak| yak.name.clone())
                .collect();

            for child_name in children {
                self.storage.write_field(&child_name, STATE_FIELD, "done")?;
            }
            self.log
                .log_command(&format!("done --recursive {resolved_name}"))?;
        } else {
            // Mark as done (or undone if undo flag is set)
            let state = if undo { "todo" } else { "done" };
            self.storage
                .write_field(&resolved_name, STATE_FIELD, state)?;
            if undo {
                self.log
                    .log_command(&format!("done --undo {resolved_name}"))?;
            } else {
                self.log.log_command(&format!("done {resolved_name}"))?;
            }
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
                state: if done {
                    "done".to_string()
                } else {
                    "todo".to_string()
                },
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
            Ok(self.yaks.borrow().clone())
        }

        fn delete_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn rename_yak(&self, _from: &str, _to: &str) -> Result<()> {
            unimplemented!()
        }

        fn find_yak(&self, name: &str) -> Result<String> {
            // For tests, just return the name if it exists
            self.get_yak(name)?;
            Ok(name.to_string())
        }

        fn write_field(&self, yak_name: &str, field_name: &str, content: &str) -> Result<()> {
            use crate::domain::STATE_FIELD;
            // For tests, if writing state field, update the yak's done status
            if field_name == STATE_FIELD {
                let mut yaks = self.yaks.borrow_mut();
                if let Some(yak) = yaks.iter_mut().find(|y| y.name == yak_name) {
                    yak.state = content.to_string();
                    yak.done = content == "done";
                    Ok(())
                } else {
                    anyhow::bail!("yak '{}' not found", yak_name)
                }
            } else {
                Ok(())
            }
        }

        fn read_field(&self, _yak_name: &str, _field_name: &str) -> Result<String> {
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

    struct MockLog;

    impl LogPort for MockLog {
        fn log_command(&self, _command: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_done_yak_marks_as_done() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", false);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output, &MockLog);

        use_case.execute("test-yak", false, false).unwrap();

        assert_eq!(storage.get_yak_status("test-yak"), Some(true));
    }

    #[test]
    fn test_done_yak_with_undo_marks_as_not_done() {
        let storage = MockStorage::new();
        storage.add_yak("test-yak", true);
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output, &MockLog);

        use_case.execute("test-yak", true, false).unwrap();

        assert_eq!(storage.get_yak_status("test-yak"), Some(false));
    }

    #[test]
    fn test_done_yak_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = DoneYak::new(&storage, &output, &MockLog);

        let result = use_case.execute("nonexistent", false, false);

        assert!(result.is_err());
    }
}
