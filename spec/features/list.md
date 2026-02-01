# `yx list` - Display All Yaks

Displays all yaks with their completion status and hierarchy. Alias: `yx ls`

## Usage

```bash
yx list                              # All yaks, markdown format
yx ls                                # Same as above
yx list --format plain               # Plain text (for scripting)
yx list --only not-done              # Only incomplete yaks
yx list --only done                  # Only completed yaks
yx list --format plain --only done   # Combine options
```

## Output Formats

### Markdown (default)
```bash
- [ ] Fix the bug          # Incomplete (normal color)
- [x] Write tests          # Done (gray, ANSI \e[90m)
  - [ ] Unit tests         # Nested (2-space indent)
  - [x] Integration tests
```

Aliases: `--format markdown` or `--format md`

### Plain
```bash
Fix the bug
Write tests
Write tests/Unit tests
Write tests/Integration tests
```

No checkboxes, no colors. Nested yaks show full path. Ideal for shell completion and scripting.

Aliases: `--format plain` or `--format raw`

## Behavior

- **Sorting**: Done yaks first, then alphabetically within each level
- **Hierarchy**: Nested yaks (parent/child) indented by 2 spaces
- **Filtering**: `--only done` or `--only not-done` filters by state
- **Empty state**: "You have no yaks. Are you done?" when no yaks exist

## Examples

```bash
# Sorting example
$ yx add "zebra" && yx add "apple" && yx done "apple"
$ yx list
- [x] apple    # Done first
- [ ] zebra    # Then alphabetically

# Plain format for scripting
$ yx list --format plain --only not-done | wc -l
5  # Count incomplete yaks
```
