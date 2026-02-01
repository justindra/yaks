# `yx add` - Create New Yaks

The `add` command creates new yaks to track work items in your DAG-based TODO list.

## Purpose

The add command:
- Creates new work items (yaks) to track
- Enforces naming constraints to ensure file system compatibility
- Enables hierarchy through parent/child naming conventions

## Usage

```bash
# Add a yak with argument
yx add "Fix the bug"

# Add a multi-word yak without quotes
yx add Fix the bug

# Add a nested yak (creates parent/child relationship)
yx add "parent/child"
```

## Naming Rules

### Valid Characters
- Letters, numbers, spaces, hyphens, underscores
- Forward slash `/` for creating nested yaks (parent/child relationships)
- Multi-word names work with or without quotes

### Invalid Characters
These characters are rejected to ensure file system compatibility:
- Backslash `\`
- Colon `:`
- Asterisk `*`
- Question mark `?`
- Pipe `|`
- Less than `<`
- Greater than `>`
- Double quotes `"`

Invalid names return an error message and non-zero exit code.

## Design Decisions

### Why allow names without quotes?
The command line automatically concatenates arguments, so:
```bash
yx add this is a test
```
Creates a yak named "this is a test" - more natural for humans than requiring quotes.

### Why restrict special characters?
Yak names map directly to directory names in `.yaks/`. Forbidden characters are those that:
- Cause file system issues (backslash, colon on some systems)
- Create shell parsing ambiguity (pipe, less than, greater than, quotes)
- Conflict with glob patterns (asterisk, question mark)

This keeps the storage layer simple and portable.

### Why use forward slash for hierarchy?
The `/` character:
- Creates an intuitive parent/child syntax (`parent/child`)
- Maps naturally to nested directory structure (`.yaks/parent/child/`)
- Avoids confusion with file paths (since `.yaks/` is the root)
- Familiar to users from file systems and URLs

## Hierarchy and Nesting

Using forward slash creates parent/child relationships:
```bash
yx add "implement-feature"
yx add "implement-feature/write-tests"
yx add "implement-feature/write-code"
yx add "implement-feature/write-tests/unit-tests"
```

This creates a tree structure:
```
- [ ] implement-feature
  - [ ] write-tests
    - [ ] unit-tests
  - [ ] write-code
```

Parent yaks serve as containers - see `done.md` for how hierarchy affects completion.

## File System Implementation

Each yak becomes a directory under `.yaks/`:
```
.yaks/
  Fix-the-bug/       # Spaces converted to hyphens in directory name
    context.md       # Optional, created later with `yx context`
```

Nested yaks create nested directories:
```
.yaks/
  parent/
    child/
      context.md
```

The directory name normalization (spaces to hyphens) happens transparently - users always reference yaks by their display name.
