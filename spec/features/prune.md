# `yx prune` - Remove All Done Yaks

Removes all completed yaks in bulk. Silent operation (no output on success).

## Usage

```bash
yx prune   # Remove all done yaks, no confirmation
```

## Behavior

- Removes all yaks with `state = "done"` (including nested)
- Keeps all incomplete yaks
- Each yak evaluated independently (done child removed even if parent not done)
- Each removal logged to git ref for audit trail
- Exit code 0, no output on success

## When to Use

- `yx prune` - Bulk cleanup of all done yaks
- `yx rm <name>` - Remove specific yak (done or not done)

Use prune after completing a sprint/milestone. Consider capturing context before pruning if needed for history.

## Example

```bash
yx add "parent" && yx add "parent/child1" && yx add "parent/child2"
yx done "parent/child1"
yx prune
# Result: parent and child2 remain, child1 removed
```
