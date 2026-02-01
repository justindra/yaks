# `yx move` - Rename and Reorganize Yaks

Renames yaks and reorganizes them in the hierarchy. Alias: `yx mv`

## Usage

```bash
yx move "old name" "new name"        # Simple rename
yx mv "standalone" "parent/child"    # Nest under parent
yx mv "parent/child" "child"         # Flatten to top-level
yx mv "old/child" "new/child"        # Move between parents
```

## Behavior

- **Preserves all data**: context, state (done/todo), and children move with parent
- **Validates new name**: rejects forbidden characters (`:` etc), returns error
- **Creates parents implicitly**: moving to `parent/child` auto-creates `parent` if needed
- **Fuzzy matching**: old name uses fuzzy matching, new name validated strictly
- **Error handling**: "not found" for non-existent source

## Examples

```bash
# Organize flat yak into hierarchy
yx add "implement feature"
yx mv "implement feature" "project-x/feature"
# Creates "project-x" if it doesn't exist

# Rename while preserving context and state
yx add "task"
echo "notes" | yx context "task"
yx done "task"
yx mv "task" "better name"
# "better name" is still done, context preserved
```
