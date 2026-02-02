// PruneYaks use case - removes all done yaks

use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct PruneYaks<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> PruneYaks<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self) -> Result<()> {
        // Get all yaks
        let yaks = self.storage.list_yaks()?;

        // Filter for done yaks
        let done_yaks: Vec<_> = yaks.iter().filter(|y| y.done).collect();

        if done_yaks.is_empty() {
            self.output.info("No done yaks to prune");
            return Ok(());
        }

        // Delete each done yak
        let count = done_yaks.len();
        for yak in done_yaks {
            self.storage.delete_yak(&yak.name)?;
        }

        self.output
            .success(&format!("Pruned {} done yak{}", count, if count == 1 { "" } else { "s" }));

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
                anyhow::bail!("Yak '{}' does not exist", name)
            }
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
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_prune_removes_all_done_yaks() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        storage.add_yak("done2", true);
        storage.add_yak("active", false);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output);

        use_case.execute().unwrap();

        assert_eq!(storage.count_yaks(), 1);
        assert_eq!(storage.count_done_yaks(), 0);
    }

    #[test]
    fn test_prune_outputs_correct_count_singular() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output);

        use_case.execute().unwrap();

        assert_eq!(output.last_message(), Some("Pruned 1 done yak".to_string()));
    }

    #[test]
    fn test_prune_outputs_correct_count_plural() {
        let storage = MockStorage::new();
        storage.add_yak("done1", true);
        storage.add_yak("done2", true);
        storage.add_yak("done3", true);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output);

        use_case.execute().unwrap();

        assert_eq!(
            output.last_message(),
            Some("Pruned 3 done yaks".to_string())
        );
    }

    #[test]
    fn test_prune_handles_no_done_yaks() {
        let storage = MockStorage::new();
        storage.add_yak("active1", false);
        storage.add_yak("active2", false);
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output);

        use_case.execute().unwrap();

        assert_eq!(storage.count_yaks(), 2);
        assert_eq!(
            output.last_message(),
            Some("No done yaks to prune".to_string())
        );
    }

    #[test]
    fn test_prune_handles_empty_list() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = PruneYaks::new(&storage, &output);

        use_case.execute().unwrap();

        assert_eq!(
            output.last_message(),
            Some("No done yaks to prune".to_string())
        );
    }
}
