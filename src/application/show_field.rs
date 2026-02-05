// ShowField use case - reads and displays a custom field

use crate::ports::{LogPort, OutputPort, StoragePort};
use anyhow::Result;

pub struct ShowField<'a> {
    storage: &'a dyn StoragePort,
    output: &'a dyn OutputPort,
    log: &'a dyn LogPort,
}

impl<'a> ShowField<'a> {
    pub fn new(
        storage: &'a dyn StoragePort,
        output: &'a dyn OutputPort,
        log: &'a dyn LogPort,
    ) -> Self {
        Self {
            storage,
            output,
            log,
        }
    }

    pub fn execute(&self, yak_name: &str, field_name: &str) -> Result<()> {
        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(yak_name)?;

        // Read field content
        let content = self.storage.read_field(&resolved_name, field_name)?;

        // Output the yak name and content (similar to context --show)
        self.output
            .info(&format!("{}\n\n{}", resolved_name, content));

        // Log the command
        self.log
            .log_command(&format!("field {resolved_name} {field_name} --show"))?;

        Ok(())
    }
}
