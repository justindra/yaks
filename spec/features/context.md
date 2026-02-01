# `yx context` - Manage Yak Context

Adds detailed notes, requirements, or background to yaks. Stored in `.yaks/<yak>/context.md`

## Usage

```bash
yx context "my yak"                  # Edit interactively ($EDITOR or vi)
echo "details" | yx context "my yak" # Set from stdin (overwrites)
yx context --show "my yak"           # Display yak + context
```

## Behavior

**Edit mode** (default):
- Interactive (terminal): Opens `$EDITOR` or `vi`
- Pipeline (stdin): Reads from stdin, overwrites existing context

**Show mode** (`--show`):
- Displays yak name + blank line + context (if any)
- If no context exists, shows only name

**Context replacement**: Stdin input replaces (doesn't append) existing context

## When to Use

Use context for requirements, acceptance criteria, technical notes, links, or collaboration details. Keep yak names short, context detailed.

## Examples

```bash
# Set via stdin
echo "## Requirements\n- Must support dark mode" | yx context "feature"

# Show
yx context --show "feature"
# Output:
# feature
#
# ## Requirements
# - Must support dark mode

# Interactive editing
yx context "feature"  # Opens editor
```
