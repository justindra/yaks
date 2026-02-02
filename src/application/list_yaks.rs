// ListYaks use case - displays all yaks

use crate::domain::Yak;
use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;

pub struct ListYaks<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
}

impl<'a> ListYaks<'a> {
    pub fn new(storage: &'a dyn StoragePort, output: &'a dyn OutputPort) -> Self {
        Self { storage, output }
    }

    pub fn execute(&self) -> Result<()> {
        let yaks = self.storage.list_yaks()?;

        if yaks.is_empty() {
            self.output.info("You have no yaks. Are you done?");
            return Ok(());
        }

        // Sort yaks: not-done first, then done, both alphabetically
        let mut sorted_yaks = yaks;
        sorted_yaks.sort_by(|a, b| {
            match (a.done, b.done) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        for yak in sorted_yaks {
            self.display_yak(&yak);
        }

        Ok(())
    }

    fn display_yak(&self, yak: &Yak) {
        let depth = yak.name.matches('/').count();
        let indent = "  ".repeat(depth);
        let display_name = yak.name.split('/').last().unwrap_or(&yak.name);

        let checkbox = if yak.done { "[x]" } else { "[ ]" };
        let message = format!("{}- {} {}", indent, checkbox, display_name);

        if yak.done {
            // Use ANSI gray color for done yaks (matches bash version)
            self.output.info(&format!("\x1b[90m{}\x1b[0m", message));
        } else {
            self.output.info(&message);
        }
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

        fn add_yak(&self, yak: Yak) {
            self.yaks.borrow_mut().push(yak);
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
    fn test_list_empty_yaks() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute().unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "You have no yaks. Are you done?");
    }

    #[test]
    fn test_list_single_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak(Yak::new("test-yak".to_string()));
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute().unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "- [ ] test-yak");
    }

    #[test]
    fn test_list_sorts_done_last() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak(Yak::new("done-yak".to_string()).mark_done());
        storage.add_yak(Yak::new("active-yak".to_string()));
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute().unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "- [ ] active-yak");
        // Second message should be grayed out and have [x]
        assert!(messages[1].contains("[x]"));
        assert!(messages[1].contains("done-yak"));
    }

    #[test]
    fn test_list_hierarchical_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak(Yak::new("parent/child".to_string()));
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute().unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "  - [ ] child");
    }
}
