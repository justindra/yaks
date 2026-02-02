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

    pub fn execute(&self, format: &str, only: Option<&str>) -> Result<()> {
        let yaks = self.storage.list_yaks()?;

        // Apply filter if specified
        let filtered_yaks: Vec<Yak> = match only {
            Some("done") => yaks.into_iter().filter(|y| y.done).collect(),
            Some("not-done") => yaks.into_iter().filter(|y| !y.done).collect(),
            _ => yaks,
        };

        // Normalize format (treat "md" and "raw" as aliases)
        let normalized_format = match format {
            "md" => "markdown",
            "raw" => "plain",
            other => other,
        };

        if filtered_yaks.is_empty() {
            // Only show message in markdown format
            if normalized_format == "markdown" {
                self.output.info("You have no yaks. Are you done?");
            }
            return Ok(());
        }

        // Sort yaks: done first, then not-done, both alphabetically
        let mut sorted_yaks = filtered_yaks;
        sorted_yaks.sort_by(|a, b| {
            match (a.done, b.done) {
                (true, false) => std::cmp::Ordering::Less,   // done before not-done
                (false, true) => std::cmp::Ordering::Greater, // not-done after done
                _ => a.name.cmp(&b.name),                     // same status: alphabetical
            }
        });

        for yak in sorted_yaks {
            self.display_yak(&yak, normalized_format);
        }

        Ok(())
    }

    fn display_yak(&self, yak: &Yak, format: &str) {
        let message = match format {
            "plain" => {
                // Plain format shows full yak name without formatting
                yak.name.clone()
            }
            _ => {
                // Markdown format (default) with hierarchy
                let depth = yak.name.matches('/').count();
                let indent = "  ".repeat(depth);
                let display_name = yak.name.split('/').last().unwrap_or(&yak.name);
                let checkbox = if yak.done { "[x]" } else { "[ ]" };
                format!("{}- {} {}", indent, checkbox, display_name)
            }
        };

        // Apply gray color for done yaks in markdown format
        if yak.done && format == "markdown" {
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

        use_case.execute("markdown", None).unwrap();

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

        use_case.execute("markdown", None).unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "- [ ] test-yak");
    }

    #[test]
    fn test_list_sorts_done_first() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak(Yak::new("done-yak".to_string()).mark_done());
        storage.add_yak(Yak::new("active-yak".to_string()));
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute("markdown", None).unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 2);
        // First message should be grayed out and have [x] (done yaks come first)
        assert!(messages[0].contains("[x]"));
        assert!(messages[0].contains("done-yak"));
        assert_eq!(messages[1], "- [ ] active-yak");
    }

    #[test]
    fn test_list_hierarchical_yak() {
        let storage = MockStorage::new();
        let output = MockOutput::new();
        storage.add_yak(Yak::new("parent/child".to_string()));
        let use_case = ListYaks::new(&storage, &output);

        use_case.execute("markdown", None).unwrap();

        let messages = output.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "  - [ ] child");
    }
}
