// SetState use case - sets the state of a yak

use crate::domain::STATE_FIELD;
use crate::ports::{LogPort, OutputPort, StoragePort};
use anyhow::Result;

pub struct SetState<'a> {
    storage: &'a dyn StoragePort,
    log: &'a dyn LogPort,
}

impl<'a> SetState<'a> {
    pub fn new(
        storage: &'a dyn StoragePort,
        _output: &'a dyn OutputPort,
        log: &'a dyn LogPort,
    ) -> Self {
        Self { storage, log }
    }

    pub fn execute(&self, name: &str, state: &str) -> Result<()> {
        // Validate state
        const VALID_STATES: &[&str] = &["todo", "wip", "done"];
        if !VALID_STATES.contains(&state) {
            anyhow::bail!(
                "Invalid state '{}'. Valid states are: todo, wip, done",
                state
            );
        }

        // Resolve yak name (exact or fuzzy match)
        let resolved_name = self.storage.find_yak(name)?;

        // Set the state
        self.storage
            .write_field(&resolved_name, STATE_FIELD, state)?;

        // If child state changes from "todo", set all parents to "wip"
        if state != "todo" {
            self.set_parents_to_wip(&resolved_name)?;
        }

        // Log the state change
        self.log
            .log_command(&format!("Set state of '{resolved_name}' to '{state}'"))?;

        Ok(())
    }

    fn set_parents_to_wip(&self, yak_name: &str) -> Result<()> {
        // Get all parent yak names from the hierarchy
        let parts: Vec<&str> = yak_name.split('/').collect();

        // Build parent paths (e.g., "a/b/c" has parents "a" and "a/b")
        for i in 1..parts.len() {
            let parent_name = parts[..i].join("/");

            // Get parent yak
            if let Ok(parent) = self.storage.get_yak(&parent_name) {
                // Only set to wip if not already at a different state
                if parent.state == "todo" {
                    self.storage.write_field(&parent_name, STATE_FIELD, "wip")?;
                }
            }
        }

        Ok(())
    }
}
