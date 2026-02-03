# Pretty CLI Format Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace markdown format as default with a pretty format using Unicode box-drawing and colored status indicators.

**Architecture:** Extend the existing format switch in `list_yaks.rs` to add "pretty" format. The tree structure logic already exists; we're enhancing visual rendering with prefix tracking for box-drawing characters and ANSI color codes.

**Tech Stack:** Rust, existing list_yaks.rs infrastructure, ANSI escape codes

---

## Task 1: Create Golden Master Test Fixtures Directory

**Files:**
- Create: `tests/fixtures/` directory
- Create: `tests/fixtures/README.md`

**Step 1: Create fixtures directory**

Run: `mkdir -p tests/fixtures`

**Step 2: Create README documenting golden master approach**

Create `tests/fixtures/README.md`:

```markdown
# Golden Master Test Fixtures

This directory contains expected output for visual format tests.

Files include ANSI escape codes for colors and formatting.

## Viewing Files

To see the actual colored output:
```bash
cat <filename> | less -R
```

## Updating Golden Masters

When intentionally changing output format:
1. Review the new output manually
2. Update the golden file
3. Commit with explanation of visual change
```

**Step 3: Commit**

```bash
git add tests/fixtures/
git commit -m "Add golden master test fixtures directory"
```

---

## Task 2: Add Helper Type for Tree Prefix Tracking

**Files:**
- Modify: `src/application/list_yaks.rs:8-14` (after YakNode struct)

**Step 1: Write test for prefix building**

Add to `#[cfg(test)] mod tests` section at end of file (around line 363):

```rust
#[test]
fn test_tree_prefix_for_middle_child() {
    let prefix = TreePrefix::new();
    let child_prefix = prefix.for_child(false);
    assert_eq!(child_prefix.get_connector(), "├─ ");
    assert_eq!(child_prefix.get_continuation(), "│  ");
}

#[test]
fn test_tree_prefix_for_last_child() {
    let prefix = TreePrefix::new();
    let child_prefix = prefix.for_child(true);
    assert_eq!(child_prefix.get_connector(), "╰─ ");
    assert_eq!(child_prefix.get_continuation(), "   ");
}

#[test]
fn test_tree_prefix_nesting() {
    let root = TreePrefix::new();
    let child = root.for_child(false); // middle child
    let grandchild = child.for_child(true); // last child of middle
    assert_eq!(grandchild.get_full_prefix(), "│  ╰─ ");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test tree_prefix`
Expected: FAIL with "cannot find type `TreePrefix`"

**Step 3: Implement TreePrefix struct**

Add after `YakNode` struct (around line 14):

```rust
/// Tracks tree drawing state for pretty format
#[derive(Clone)]
struct TreePrefix {
    /// Accumulated prefix lines from parent levels
    lines: Vec<String>,
}

impl TreePrefix {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// Create prefix for a child node
    fn for_child(&self, is_last: bool) -> Self {
        let mut new_lines = self.lines.clone();
        let continuation = if is_last { "   " } else { "│  " };
        new_lines.push(continuation.to_string());
        Self { lines: new_lines }
    }

    /// Get the connector for this level (├─ or ╰─)
    fn get_connector(&self) -> &str {
        if self.lines.is_empty() {
            ""
        } else if self.lines.last().unwrap() == "   " {
            "╰─ "
        } else {
            "├─ "
        }
    }

    /// Get the continuation line for children
    fn get_continuation(&self) -> &str {
        if self.lines.is_empty() {
            ""
        } else {
            self.lines.last().unwrap()
        }
    }

    /// Build full prefix string for displaying this node
    fn get_full_prefix(&self) -> String {
        if self.lines.is_empty() {
            String::new()
        } else {
            let parent_lines = &self.lines[..self.lines.len() - 1];
            let connector = self.get_connector();
            format!("{}{}", parent_lines.join(""), connector)
        }
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test tree_prefix`
Expected: All 3 tests PASS

**Step 5: Commit**

```bash
git add src/application/list_yaks.rs
git commit -m "Add TreePrefix helper for box-drawing chars"
```

---

## Task 3: Extend display_tree to Track Last Child Status

**Files:**
- Modify: `src/application/list_yaks.rs:156-177` (display_tree method)

**Step 1: Write test for tree rendering with prefixes**

Add to tests section:

```rust
#[test]
fn test_display_tree_tracks_last_child() {
    let storage = MockStorage::new();
    let output = MockOutput::new();
    storage.add_yak(Yak::new("parent/first".to_string()));
    storage.add_yak(Yak::new("parent/last".to_string()));
    let use_case = ListYaks::new(&storage, &output);

    use_case.execute("pretty", None).unwrap();

    let messages = output.get_messages();
    // First child should have middle connector
    assert!(messages.iter().any(|m| m.contains("├─")));
    // Last child should have last connector
    assert!(messages.iter().any(|m| m.contains("╰─")));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test display_tree_tracks_last_child`
Expected: FAIL (pretty format not yet implemented)

**Step 3: Update display_tree signature to accept TreePrefix**

Change `display_tree` method signature (around line 157):

```rust
fn display_tree(
    &self,
    nodes: &[YakNode],
    format: &str,
    only: Option<&str>,
    prefix: &TreePrefix,
    has_output: &mut bool,
) {
```

**Step 4: Update display_tree to track last child**

Replace the loop in `display_tree` (around line 165):

```rust
for (i, node) in nodes.iter().enumerate() {
    let is_last = i == nodes.len() - 1;

    // Check if node should be displayed based on filter
    let should_display = self.should_display_node(node, only);

    if should_display {
        *has_output = true;
        self.display_node(node, format, prefix, is_last);
    }

    // Recurse to children with updated prefix
    let child_prefix = prefix.for_child(is_last);
    self.display_tree(&node.children, format, only, &child_prefix, has_output);
}
```

**Step 5: Update display_node signature**

Change `display_node` signature (around line 191):

```rust
fn display_node(&self, node: &YakNode, format: &str, prefix: &TreePrefix, is_last: bool) {
```

**Step 6: Update call site in execute method**

Update the call to `display_tree` in `execute` method (around line 49):

```rust
let root_prefix = TreePrefix::new();
self.display_tree(&tree, normalized_format, only, &root_prefix, &mut has_output);
```

**Step 7: Run tests to check compile errors**

Run: `cargo test`
Expected: Compile errors about display_node needing updates

**Step 8: Commit**

```bash
git add src/application/list_yaks.rs
git commit -m "Thread TreePrefix through display_tree"
```

---

## Task 4: Implement Pretty Format Display Logic

**Files:**
- Modify: `src/application/list_yaks.rs:191-209` (display_node method)

**Step 1: Create golden master for single yak**

Create `tests/fixtures/pretty_single_yak.golden`:

```
○ test-yak
```

**Step 2: Write test comparing to golden master**

Add to tests:

```rust
#[test]
fn test_pretty_format_single_yak() {
    let storage = MockStorage::new();
    let output = MockOutput::new();
    storage.add_yak(Yak::new("test-yak".to_string()));
    let use_case = ListYaks::new(&storage, &output);

    use_case.execute("pretty", None).unwrap();

    let actual = output.get_messages().join("\n");
    let expected = include_str!("../../tests/fixtures/pretty_single_yak.golden").trim();
    assert_eq!(actual, expected);
}
```

**Step 3: Run test to verify it fails**

Run: `cargo test pretty_format_single_yak`
Expected: FAIL (pretty format not implemented)

**Step 4: Implement pretty format in display_node**

Replace the `display_node` method (starting around line 191):

```rust
fn display_node(&self, node: &YakNode, format: &str, prefix: &TreePrefix, is_last: bool) {
    let message = match format {
        "plain" => node.full_path.clone(),
        "pretty" => {
            let tree_prefix = prefix.get_full_prefix();
            let is_done = node.yak.as_ref().map(|y| y.done).unwrap_or(false);
            let status_dot = if is_done { "●" } else { "○" };

            if is_done {
                // Dimmed and strikethrough for done yaks
                format!("\x1b[2;9m{}{} {}\x1b[0m", tree_prefix, status_dot, node.name)
            } else {
                // Normal for active yaks
                format!("{}{} {}", tree_prefix, status_dot, node.name)
            }
        }
        _ => {
            // markdown format (existing logic)
            let indent = "  ".repeat(prefix.lines.len());
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
```

**Step 5: Run test to verify it passes**

Run: `cargo test pretty_format_single_yak`
Expected: PASS

**Step 6: Commit**

```bash
git add src/application/list_yaks.rs tests/fixtures/pretty_single_yak.golden
git commit -m "Implement pretty format with status dots"
```

---

## Task 5: Add Golden Master Test for Hierarchy

**Files:**
- Create: `tests/fixtures/pretty_hierarchy.golden`
- Modify: `src/application/list_yaks.rs` (add test)

**Step 1: Create golden master for hierarchy**

Create `tests/fixtures/pretty_hierarchy.golden`:

```
○ parent
├─ ○ first-child
│  ╰─ ○ grandchild
╰─ ○ last-child
```

**Step 2: Write test for hierarchy**

Add to tests:

```rust
#[test]
fn test_pretty_format_hierarchy() {
    let storage = MockStorage::new();
    let output = MockOutput::new();
    storage.add_yak(Yak::new("parent/first-child/grandchild".to_string()));
    storage.add_yak(Yak::new("parent/last-child".to_string()));
    let use_case = ListYaks::new(&storage, &output);

    use_case.execute("pretty", None).unwrap();

    let actual = output.get_messages().join("\n");
    let expected = include_str!("../../tests/fixtures/pretty_hierarchy.golden").trim();
    assert_eq!(actual, expected);
}
```

**Step 3: Run test to verify behavior**

Run: `cargo test pretty_format_hierarchy`
Expected: Should PASS if prefix logic is correct, may need debugging

**Step 4: Debug and fix if needed**

If test fails, check:
- TreePrefix logic for nested levels
- get_full_prefix building correct string
- Sorting order of children

**Step 5: Commit**

```bash
git add tests/fixtures/pretty_hierarchy.golden src/application/list_yaks.rs
git commit -m "Add golden test for hierarchy rendering"
```

---

## Task 6: Add Golden Master Test for Done Yaks

**Files:**
- Create: `tests/fixtures/pretty_with_done.golden`
- Modify: `src/application/list_yaks.rs` (add test)

**Step 1: Create golden master with done yaks**

Create `tests/fixtures/pretty_with_done.golden` (with actual ANSI codes):

```
[2;9m● done-yak[0m
○ active-yak
○ parent
├─ [2;9m● done-child[0m
╰─ ○ active-child
```

**Step 2: Write test for done yaks**

Add to tests:

```rust
#[test]
fn test_pretty_format_with_done() {
    let storage = MockStorage::new();
    let output = MockOutput::new();
    storage.add_yak(Yak::new("done-yak".to_string()).mark_done());
    storage.add_yak(Yak::new("active-yak".to_string()));
    storage.add_yak(Yak::new("parent/done-child".to_string()).mark_done());
    storage.add_yak(Yak::new("parent/active-child".to_string()));
    let use_case = ListYaks::new(&storage, &output);

    use_case.execute("pretty", None).unwrap();

    let actual = output.get_messages().join("\n");
    let expected = include_str!("../../tests/fixtures/pretty_with_done.golden").trim();
    assert_eq!(actual, expected);
}
```

**Step 3: Run test to verify behavior**

Run: `cargo test pretty_format_with_done`
Expected: Should PASS if done formatting is correct

**Step 4: Adjust ANSI codes if needed**

If test fails due to ANSI code differences:
- Check the actual output with: `cargo test pretty_format_with_done -- --nocapture`
- Update golden file with correct codes
- Verify visually that dimming and strikethrough work

**Step 5: Commit**

```bash
git add tests/fixtures/pretty_with_done.golden src/application/list_yaks.rs
git commit -m "Add golden test for done yak rendering"
```

---

## Task 7: Change Default Format to Pretty

**Files:**
- Modify: `src/main.rs:36` (default_value in List command)

**Step 1: Write integration test for default format**

Add to `tests/integration_test.rs`:

```rust
#[test]
fn test_default_format_is_pretty() {
    let tmp_dir = TempDir::new().unwrap();
    let yak_path = tmp_dir.path().to_str().unwrap();

    // Add a yak
    Command::new(env!("CARGO_BIN_EXE_yx"))
        .env("YAK_PATH", yak_path)
        .args(&["add", "test-yak"])
        .output()
        .unwrap();

    // List without format flag
    let output = Command::new(env!("CARGO_BIN_EXE_yx"))
        .env("YAK_PATH", yak_path)
        .args(&["ls"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should have pretty format dot, not markdown checkbox
    assert!(stdout.contains("○"));
    assert!(!stdout.contains("[ ]"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_default_format_is_pretty`
Expected: FAIL (default is still markdown)

**Step 3: Change default format in main.rs**

Change line 36 in `src/main.rs`:

```rust
/// Output format (markdown, md, plain, raw, pretty)
#[arg(long, default_value = "pretty")]
format: String,
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_default_format_is_pretty`
Expected: PASS

**Step 5: Run all tests**

Run: `cargo test`
Expected: All tests PASS

**Step 6: Commit**

```bash
git add src/main.rs tests/integration_test.rs
git commit -m "Change default format to pretty"
```

---

## Task 8: Update Help Text and Documentation

**Files:**
- Modify: `src/main.rs:35-37` (List command format arg)
- Modify: `README.md` (if it documents format flag)

**Step 1: Update format argument help text**

Change the format argument documentation in `src/main.rs` (around line 35):

```rust
/// Output format: pretty (default), markdown, plain
/// - pretty: Unicode box-drawing with colored status dots
/// - markdown: Checkbox-style list with indentation
/// - plain: Just yak names, one per line
#[arg(long, default_value = "pretty")]
format: String,
```

**Step 2: Check if README mentions format flag**

Run: `grep -n "format" README.md`

If found, update the relevant section to mention pretty as default.

**Step 3: Build and test help output**

Run: `cargo build && ./target/debug/yx list --help`
Expected: Help text shows pretty as default with description

**Step 4: Commit**

```bash
git add src/main.rs README.md
git commit -m "Update help text for pretty format default"
```

---

## Task 9: Manual Visual Testing

**Files:**
- None (manual testing)

**Step 1: Build release binary**

Run: `cargo build --release`

**Step 2: Test with actual project yaks**

Run: `YAK_PATH=../../.yaks ./target/release/yx ls`

Verify visually:
- Box-drawing characters render correctly
- Tree structure is clear
- Done yaks are dimmed and struck through
- Colors look good

**Step 3: Test other formats still work**

Run: `YAK_PATH=../../.yaks ./target/release/yx ls --format markdown`
Expected: Old checkbox format

Run: `YAK_PATH=../../.yaks ./target/release/yx ls --format plain`
Expected: Just yak names

**Step 4: Test with various terminals**

If possible, test in:
- Current terminal
- iTerm2 / Terminal.app
- tmux session

**Step 5: Document any issues**

If any visual issues found, create follow-up tasks.

---

## Task 10: Run Full Test Suite and Quality Checks

**Files:**
- None (verification)

**Step 1: Run all Rust tests**

Run: `cargo test`
Expected: All tests PASS

**Step 2: Run ShellSpec tests**

Run: `shellspec`
Expected: All specs PASS (existing behavior unchanged)

**Step 3: Run linting**

Run: `dev lint`
Expected: No warnings or errors

**Step 4: Run full quality checks**

Run: `dev check`
Expected: All checks PASS

**Step 5: Review git log**

Run: `git log --oneline`
Expected: Clean, incremental commit history

---

## Success Criteria

- [ ] All unit tests pass including golden master tests
- [ ] All integration tests pass
- [ ] ShellSpec tests still pass (backward compatibility)
- [ ] `dev check` passes (lint + test + audit)
- [ ] Default format is pretty
- [ ] All existing formats still work
- [ ] Visual output looks good in terminal
- [ ] Clean commit history with 10 incremental commits

## Notes

- The `TreePrefix` struct is the key abstraction for tracking box-drawing state
- Golden master tests capture exact ANSI output for regression testing
- Existing markdown/plain formats remain unchanged for backward compatibility
- Manual visual testing is important since golden masters only test string content
