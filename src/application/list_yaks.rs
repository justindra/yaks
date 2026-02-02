// ListYaks use case - displays all yaks

use crate::domain::Yak;
use crate::ports::{OutputPort, StoragePort};
use anyhow::Result;
use std::collections::HashMap;

/// Represents a node in the yak hierarchy tree
struct YakNode {
    name: String,      // Just the leaf name (e.g., "child" not "parent/child")
    full_path: String, // Full path (e.g., "parent/child")
    yak: Option<Yak>,  // None for implicit parents
    children: Vec<YakNode>,
}

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

        // Normalize format (treat "md" and "raw" as aliases)
        let normalized_format = match format {
            "md" => "markdown",
            "raw" => "plain",
            other => other,
        };

        if yaks.is_empty() {
            // Only show message in markdown format
            if normalized_format == "markdown" {
                self.output.info("You have no yaks. Are you done?");
            }
            return Ok(());
        }

        // Build hierarchy tree
        let tree = self.build_tree(yaks);

        // Display tree with filtering
        let mut has_output = false;
        self.display_tree(&tree, normalized_format, only, 0, &mut has_output);

        // If filtered and nothing to show
        if !has_output && normalized_format == "markdown" {
            self.output.info("You have no yaks. Are you done?");
        }

        Ok(())
    }

    /// Build a hierarchical tree from flat list of yaks
    fn build_tree(&self, yaks: Vec<Yak>) -> Vec<YakNode> {
        let mut nodes_by_path: HashMap<String, YakNode> = HashMap::new();

        // First pass: create nodes for all yaks and implicit parents
        for yak in &yaks {
            let parts: Vec<&str> = yak.name.split('/').collect();

            // Create implicit parent nodes if they don't exist
            for i in 1..parts.len() {
                let parent_path = parts[..i].join("/");
                if !nodes_by_path.contains_key(&parent_path) {
                    let parent_name = parts[i - 1].to_string();
                    nodes_by_path.insert(
                        parent_path.clone(),
                        YakNode {
                            name: parent_name,
                            full_path: parent_path.clone(),
                            yak: None, // Implicit parent (no actual yak)
                            children: Vec::new(),
                        },
                    );
                }
            }

            // Create node for this yak
            let name = parts.last().unwrap_or(&"").to_string();
            nodes_by_path.insert(
                yak.name.clone(),
                YakNode {
                    name,
                    full_path: yak.name.clone(),
                    yak: Some(yak.clone()),
                    children: Vec::new(),
                },
            );
        }

        // Second pass: build parent-child relationships
        // Sort paths by depth (deepest first) to ensure children are processed before parents
        let mut all_paths: Vec<String> = nodes_by_path.keys().cloned().collect();
        all_paths.sort_by_key(|p| std::cmp::Reverse(p.matches('/').count()));

        // Extract children from deepest to shallowest
        for path in &all_paths {
            let parts: Vec<&str> = path.split('/').collect();

            if parts.len() == 1 {
                // Root node - leave it
                continue;
            }

            // Child node - attach to parent
            let parent_path = parts[..parts.len() - 1].join("/");

            // Remove child from map and attach to parent
            if let Some(child_node) = nodes_by_path.remove(path) {
                if let Some(parent_node) = nodes_by_path.get_mut(&parent_path) {
                    parent_node.children.push(child_node);
                } else {
                    // This shouldn't happen since we created all parents in first pass
                    // But if it does, put the node back
                    nodes_by_path.insert(path.clone(), child_node);
                }
            }
        }

        // Extract root nodes and sort
        let mut roots: Vec<YakNode> = nodes_by_path
            .into_iter()
            .filter(|(path, _)| !path.contains('/'))
            .map(|(_, node)| node)
            .collect();

        Self::sort_children(&mut roots);
        roots
    }

    /// Sort children at this level: done first, then not-done, both alphabetically
    fn sort_children(children: &mut [YakNode]) {
        children.sort_by(|a, b| {
            let a_done = a.yak.as_ref().map(|y| y.done).unwrap_or(false);
            let b_done = b.yak.as_ref().map(|y| y.done).unwrap_or(false);

            match (a_done, b_done) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        // Recursively sort children's children
        for child in children.iter_mut() {
            Self::sort_children(&mut child.children);
        }
    }

    /// Display tree recursively
    fn display_tree(
        &self,
        nodes: &[YakNode],
        format: &str,
        only: Option<&str>,
        depth: usize,
        has_output: &mut bool,
    ) {
        for node in nodes {
            // Check if node should be displayed based on filter
            let should_display = self.should_display_node(node, only);

            if should_display {
                *has_output = true;
                self.display_node(node, format, depth);
            }

            // Always recurse to children (they might be visible even if parent is filtered)
            self.display_tree(&node.children, format, only, depth + 1, has_output);
        }
    }

    /// Check if node matches the filter
    fn should_display_node(&self, node: &YakNode, only: Option<&str>) -> bool {
        match only {
            Some("done") => node.yak.as_ref().map(|y| y.done).unwrap_or(false),
            Some("not-done") => {
                !node.yak.as_ref().map(|y| y.done).unwrap_or(false) || node.yak.is_none()
            }
            _ => true,
        }
    }

    /// Display a single node
    fn display_node(&self, node: &YakNode, format: &str, depth: usize) {
        let message = match format {
            "plain" => node.full_path.clone(),
            _ => {
                let indent = "  ".repeat(depth);
                let done = node.yak.as_ref().map(|y| y.done).unwrap_or(false);
                let checkbox = if done { "[x]" } else { "[ ]" };
                format!("{}- {} {}", indent, checkbox, node.name)
            }
        };

        // Apply gray color for done yaks in markdown format
        let is_done = node.yak.as_ref().map(|y| y.done).unwrap_or(false);
        if is_done && format == "markdown" {
            self.output.info(&format!("\x1b[90m{message}\x1b[0m"));
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
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "- [ ] parent");
        assert_eq!(messages[1], "  - [ ] child");
    }
}
