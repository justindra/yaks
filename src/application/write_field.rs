// WriteField use case - writes a custom field from stdin

use crate::domain::validate_field_name;
use crate::ports::{LogPort, OutputPort, StoragePort};
use anyhow::{Context, Result};
use std::io::{self, Read};

pub struct WriteField<'a> {
    storage: &'a dyn StoragePort,
    log: &'a dyn LogPort,
}

impl<'a> WriteField<'a> {
    pub fn new(
        storage: &'a dyn StoragePort,
        _output: &'a dyn OutputPort,
        log: &'a dyn LogPort,
    ) -> Self {
        Self { storage, log }
    }

    pub fn execute(&self, yak_name: &str, field_name: &str) -> Result<()> {
        // Validate field name
        validate_field_name(field_name)?;

        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(yak_name)?;

        // Read content from stdin
        let content = self.read_from_stdin()?;

        // Write field
        self.storage
            .write_field(&resolved_name, field_name, &content)?;

        // Log the command
        self.log
            .log_command(&format!("field {resolved_name} {field_name}"))?;

        Ok(())
    }

    fn read_from_stdin(&self) -> Result<String> {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Yak;
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct MockStorage {
        yaks: RefCell<Vec<Yak>>,
        fields: RefCell<HashMap<(String, String), String>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                yaks: RefCell::new(Vec::new()),
                fields: RefCell::new(HashMap::new()),
            }
        }

        #[allow(dead_code)]
        fn add_yak(&self, name: &str) {
            self.yaks.borrow_mut().push(Yak {
                name: name.to_string(),
                done: false,
                state: "todo".to_string(),
                context: None,
            });
        }

        fn get_field(&self, yak_name: &str, field_name: &str) -> Option<String> {
            self.fields
                .borrow()
                .get(&(yak_name.to_string(), field_name.to_string()))
                .cloned()
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

        fn delete_yak(&self, _name: &str) -> Result<()> {
            unimplemented!()
        }

        fn rename_yak(&self, _from: &str, _to: &str) -> Result<()> {
            unimplemented!()
        }

        fn find_yak(&self, name: &str) -> Result<String> {
            self.get_yak(name)?;
            Ok(name.to_string())
        }

        fn write_field(&self, yak_name: &str, field_name: &str, content: &str) -> Result<()> {
            self.fields.borrow_mut().insert(
                (yak_name.to_string(), field_name.to_string()),
                content.to_string(),
            );
            Ok(())
        }

        fn read_field(&self, yak_name: &str, field_name: &str) -> Result<String> {
            self.get_field(yak_name, field_name)
                .ok_or_else(|| anyhow::anyhow!("Field not found"))
        }
    }

    struct MockOutput;

    impl OutputPort for MockOutput {
        fn success(&self, _message: &str) {}
        fn error(&self, _message: &str) {}
        fn info(&self, _message: &str) {}
    }

    struct MockLog;

    impl LogPort for MockLog {
        fn log_command(&self, _command: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_write_field_fails_for_nonexistent_yak() {
        let storage = MockStorage::new();
        let output = MockOutput;
        let log = MockLog;
        let use_case = WriteField::new(&storage, &output, &log);

        let result = use_case.execute("nonexistent", "field");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
