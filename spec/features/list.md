# `yx list` - Display All Yaks

The `list` command displays all yaks in your TODO list, showing their completion status and hierarchical relationships.

## Purpose

The list command:
- Displays all yaks with visual indicators for completion status
- Shows parent/child hierarchies through indentation
- Supports multiple output formats for different use cases
- Filters yaks by completion status
- Provides an "empty state" message when no work exists

## Usage

```bash
# List all yaks (default markdown format)
yx list

# Use the shorter alias
yx ls

# Output plain text for scripting
yx list --format plain

# Show only incomplete yaks
yx list --only not-done

# Show only completed yaks
yx list --only done

# Combine format and filter options
yx list --format plain --only not-done
```

## Output Formats

### Markdown (default)
The default format displays yaks as GitHub-flavored markdown task lists:

```bash
$ yx list
- [ ] Fix the bug
- [x] Write tests
  - [ ] Unit tests
  - [x] Integration tests
```

**Visual indicators:**
- Incomplete yaks: `- [ ] name` in normal color
- Completed yaks: `- [x] name` in gray (ANSI `\e[90m`)
- Nested yaks: Indented by 2 spaces per level

**Aliases:** `--format markdown` or `--format md`

### Plain
Plain format outputs yak names one per line with no decoration:

```bash
$ yx list --format plain
Fix the bug
Write tests
Write tests/Unit tests
Write tests/Integration tests
```

**Key differences:**
- No checkboxes or color codes
- Nested yaks show full path (e.g., `parent/child`)
- Ideal for shell completion and scripting

**Aliases:** `--format plain` or `--format raw`

## Filtering Options

### Show only incomplete yaks
```bash
$ yx list --only not-done
- [ ] Fix the bug
  - [ ] Unit tests
```

Filters out all completed yaks, useful for focusing on remaining work.

### Show only completed yaks
```bash
$ yx list --only done
- [x] Write tests
  - [x] Integration tests
```

Filters out incomplete yaks, useful for reviewing what's been accomplished.

### Show all (default)
Without `--only`, both completed and incomplete yaks are shown.

## Sorting Behavior

Within each level of hierarchy, yaks are sorted:
1. **Done yaks first** - Completed work appears at the top
2. **Then alphabetically** - Incomplete yaks sort by name

Example:
```bash
$ yx add "zebra"
$ yx add "apple"
$ yx add "mango"
$ yx done "apple"
$ yx list
- [x] apple     # Done, so appears first
- [ ] mango     # Alphabetically before "zebra"
- [ ] zebra     # Alphabetically last
```

## Hierarchy Display

Nested yaks (created with `/` in the name) display with indentation:

```bash
$ yx add "Implement feature"
$ yx add "Implement feature/Write tests"
$ yx add "Implement feature/Write tests/Unit tests"
$ yx list
- [ ] Implement feature
  - [ ] Write tests
    - [ ] Unit tests
```

Each level adds 2 spaces of indentation. Hierarchy is preserved even when children are marked done.

## Empty State

When no yaks exist, list displays an encouraging message:

```bash
$ yx list
You have no yaks. Are you done?
```

In plain format with no yaks, nothing is output (exit code 0).

## Design Decisions

### Why show done yaks by default?
Unlike many TODO tools that hide completed items, yaks keeps them visible to:
- Provide context about what was recently accomplished
- Help teams understand the full picture of work
- Avoid confusion when a yak "disappears"

Use `yx prune` to remove done yaks when you want a clean slate.

### Why sort done yaks first?
Placing completed yaks at the top of each level:
- Makes them visible but not visually blocking
- Groups related completed work together
- Keeps the "active" work (incomplete yaks) together for easy scanning

### Why sort alphabetically?
Alphabetical sorting provides:
- **Predictability**: Yaks always appear in the same order
- **Findability**: Easy to scan for a specific yak name
- **Simplicity**: No need to track timestamps or modification times
- **Consistency**: Works reliably across sync operations and worktrees

The git-based storage makes it difficult to preserve meaningful timestamps (since `git archive` doesn't preserve directory mtimes), so alphabetical sorting is the most reliable approach.

### Why provide plain format?
The plain format serves specific use cases:
- **Shell completion**: Commands like `yx done <TAB>` need simple yak names
- **Scripting**: Other tools can parse plain output easily
- **Piping**: `yx list --format plain | grep pattern` works cleanly

### Why include full paths in plain format?
For nested yaks, plain format outputs `parent/child` instead of just `child` because:
- Uniquely identifies each yak (multiple parents could have a "child" named the same)
- Matches how users reference nested yaks in other commands
- Enables scripts to operate on specific nested yaks unambiguously

## File System Implementation

The list command traverses the `.yaks/` directory tree:
- Each directory is a yak
- A `state` file contains either "todo" or "done"
- Directory structure determines hierarchy
- Directory names determine alphabetical sort order

No index or database is needed - the file system IS the data model.
