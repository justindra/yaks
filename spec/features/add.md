# `yx add` - Create New Yaks

Creates new work items to track.

## Usage

```bash
yx add "Fix the bug"        # Quotes optional
yx add Fix the bug          # Same as above
yx add "parent/child"       # Create nested yak (hierarchy)
```

## Naming Rules

**Valid**: Letters, numbers, spaces, hyphens, underscores, forward slash `/` (for nesting)

**Invalid**: `\ : * ? | < > "` (file system compatibility)

Invalid names return error with non-zero exit code.

## Hierarchy

Forward slash creates parent/child relationships:

```bash
yx add "feature"
yx add "feature/tests"
yx add "feature/tests/unit"

# Result:
# - [ ] feature
#   - [ ] tests
#     - [ ] unit
```

See `done.md` for how hierarchy affects completion rules.
