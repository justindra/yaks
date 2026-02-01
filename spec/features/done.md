# `yx done` - Mark Yaks as Complete

Marks yaks as completed, changing their visual representation to grayed out `[x]`.

## Usage

```bash
yx done "Fix the bug"            # Mark complete
yx done --undo "Fix the bug"     # Reopen (undo)
yx done --recursive "parent"     # Mark parent + all descendants
```

## Behavior

- **Visual**: Done yaks appear as `- [x] name` in gray (ANSI `\e[90m`)
- **Hierarchy rule**: Cannot mark parent done if children are incomplete
- **Children**: Can be marked done independently
- **Recursive**: `--recursive` marks entire subtree (parent + all children/grandchildren)
- **Undo**: `--undo` reopens a done yak

## Examples

```bash
# Hierarchy enforcement
yx add "parent" && yx add "parent/child"
yx done "parent/child"   # ✓ Allowed
yx done "parent"          # ✗ Error: has incomplete children

# Recursive completion
yx done --recursive "parent"  # Marks parent and all descendants

# Undo
yx done "task"
yx done --undo "task"    # Reopens the yak
```

Done yaks stay visible in `yx list` for context. Use `yx prune` to remove them.
