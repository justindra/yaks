# `yx rm` - Remove Yaks

Deletes a yak from the working tree. The deletion is git-tracked, so yaks can be recovered from git history if needed.

## Usage

```bash
yx rm "Fix the bug"    # Quotes optional
yx rm Fix the bug      # Same as above
yx rm "parent/child"   # Remove nested yak
```

## Behavior

- Silent operation (no output on success, exit code 0)
- Returns "not found" error for non-existent yaks
- Uses fuzzy matching for name resolution
- Removes only the specified yak (not its parent or children)

## When to Use

- `yx rm <name>` - Remove a specific yak (done or not done)
- `yx prune` - Remove all done yaks in bulk

Use `rm` for mistakes or irrelevant work. Use `prune` for bulk cleanup of completed yaks.
