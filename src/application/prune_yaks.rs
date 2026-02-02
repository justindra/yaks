// PruneYaks use case - removes all done yaks

use crate::ports::{LogPort, OutputPort, StoragePort};
use anyhow::Result;

pub struct PruneYaks<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
    log: &'a dyn LogPort,
}

impl<'a> PruneYaks<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort, log: &'a dyn LogPort) -> Self {
        Self { storage, output, log }
    }

    pub fn execute(&self) -> Result<()> {
        // Get all yaks
        let yaks = self.storage.list_yaks()?;

        // Filter for done yaks
        let done_yaks: Vec<_> = yaks.iter().filter(|y| y.done).collect();

        if done_yaks.is_empty() {
            // Silently return if no done yaks to prune (matches bash behavior)
            return Ok(());
        }

        // Delete each done yak and log as "rm" individually (matches bash behavior)
        for yak in done_yaks {
            self.storage.delete_yak(&yak.name)?;
            self.log.log_command(&format!("rm {}", yak.name))?;
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

        fn count_yaks(&self) -> usize {
            self.yaks.borrow().len()
        }

        fn count_done_yaks(&self) -> usize {
            self.yaks.borrow().iter().filter(|y| y.done).count()
        }
    }

    impl StoragePort for MockStorage {
        fn create_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn get_yak(&self, _name: &str) -> Result<Yak> {
            unimplemented!()
        }

        fn list_yaks(&self) -> Result<Vec<Yak>> {
            Ok(self.yaks.borrow().clone())
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
                anyhow::bail!("yak '{}' not found", name)
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

        fn find_yak(&self, _name: &str) -> Result<String> {
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
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    struct MockLog;

    impl LogPort for MockLog {
        fn log_command(&self, _command: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_prune_removes_all_done_yaks() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        storage.add_yak("done2", true);
        storage.add_yak("active", false);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output, &MockLog);

        use_case.execute().unwrap();

        assert_eq!(storage.count_yaks(), 1);
        assert_eq!(storage.count_done_yaks(), 0);
    }

    #[test]
    fn test_prune_is_silent_when_removing_one_yak() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output, &MockLog);

        use_case.execute().unwrap();

        // Prune should be silent (matches bash behavior)
        assert_eq!(output.last_message(), None);
        assert_eq!(storage.count_yaks(), 0);
    }

    #[test]
    fn test_prune_is_silent_when_removing_multiple_yaks() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        storage.add_yak("done2", true);
        storage.add_yak("done3", true);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output, &MockLog);

        use_case.execute().unwrap();

        // Prune should be silent (matches bash behavior)
        assert_eq!(output.last_message(), None);
        assert_eq!(storage.count_yaks(), 0);
    }

    #[test]
    fn test_prune_handles_no_done_yaks() {
        let storage = MockStorage::new();
        storage.add_yak("active1", false);
        storage.add_yak("active2", false);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output, &MockLog);

        use_case.execute().unwrap();

        assert_eq!(storage.count_yaks(), 2);
        // No message expected when no done yaks (matches bash behavior)
        assert_eq!(output.last_message(), None);
    }

    #[test]
    fn test_prune_handles_empty_list() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output, &MockLog);

        use_case.execute().unwrap();

        assert_eq!(storage.count_yaks(), 0);
        // No message expected when no yaks at all (matches bash behavior)
        assert_eq!(output.last_message(), None);
    }
}
