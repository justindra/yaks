# `yx done` - Mark Yaks as Complete

The `done` command marks yaks as completed, changing their visual representation and preventing accidental re-work.

## Purpose

The done command:
- Marks individual yaks as complete
- Changes visual display to show completion status (grayed out `[x]` checkbox)
- Enforces hierarchy rules for nested yaks
- Provides undo capability to reopen completed work

## Usage

```bash
# Mark a yak as complete
yx done "Fix the bug"

# Undo completion (reopen the yak)
yx done --undo "Fix the bug"

# Mark parent and ALL children as complete recursively
yx done --recursive "parent"
```

## Visual Representation

Done yaks appear in `yx list` as:
- Grayed out text (ANSI color code `\e[90m`)
- Checked checkbox `[x]` instead of empty `[ ]`
- Example: `- [x] Fix the bug` (shown in gray)

This visual distinction helps teams quickly identify completed work while keeping it visible for context.

## Hierarchy Rules

When working with nested yaks (parent/child relationships):

1. **Children can be marked done independently**
   ```bash
   yx add "parent"
   yx add "parent/child"
   yx done "parent/child"  # ✓ Allowed
   ```

2. **Parents cannot be marked done if children are incomplete**
   ```bash
   yx add "parent"
   yx add "parent/child"
   yx done "parent"  # ✗ Error: cannot mark 'parent' as done - it has incomplete children
   ```

3. **Recursive flag marks entire subtree**
   ```bash
   yx done --recursive "parent"  # Marks parent + all children + all grandchildren
   ```

## Design Decisions

### Why keep done yaks visible?
Unlike some TODO tools that hide completed items, yaks keeps them visible (in gray) to:
- Provide context about recently completed work
- Support teams reviewing what was accomplished
- Avoid confusion about "where did that yak go?"

Use `yx prune` to remove done yaks when you want to clean up.

### Why enforce parent/child completion order?
Preventing a parent from being marked done while children are incomplete:
- Reflects reality: you can't finish shaving the yak until all sub-yaks are shaved
- Prevents accidentally "completing" work when sub-tasks remain
- Makes `yx list` an accurate representation of actual work status

### Why allow --undo?
Mistakes happen. The `--undo` flag provides a simple escape hatch for:
- Accidentally marking the wrong yak done
- Discovering additional work is needed
- Reopening work without removing the yak entirely

## Edge Cases

### Yak names starting with 'x'
The command handles yak names like "x marks the spot" correctly, despite potential confusion with the `[x]` checkbox syntax.

### Error handling
- Non-existent yaks: Returns error message and non-zero exit code
- Parent with incomplete children: Clear error message explaining the constraint

## File System Implementation

Completion is stored in a `state` file in the yak's directory:
```
.yaks/
  Fix-the-bug/
    state         # Contains "done" or "todo"
    context.md
```

The state file contains either "todo" (default when created) or "done" (after marking complete). This simple file-based state allows easy inspection and manipulation outside the CLI if needed.
