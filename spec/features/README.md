# Feature Tests

These are **black-box CLI tests** that validate the behavior of the `yx` command from a user's perspective.

## Purpose

Feature tests:
- Test the CLI interface (commands, arguments, output format)
- Test file system effects (what gets created in `.yaks/`)
- Test git ref behavior (`refs/notes/yaks` structure)
- Are **implementation-agnostic** and can be reused across language rewrites

## Acceptance Seams

These tests couple to specific interfaces that all implementations must honor:
- CLI commands and arguments
- Output format and messages
- `.yaks/` directory structure
- `refs/notes/yaks` git ref structure for sync

Any implementation in any language must maintain these contracts.

## Running Feature Tests

```bash
# Run only feature tests
shellspec spec/features/

# Run a specific feature test
shellspec spec/features/add.sh
```

## Writing Feature Tests

Feature tests should:
- Test observable behavior through the CLI and git
- Verify output, exit codes, file system state, and git refs
- Use the `yx` command as users would
- Be independent of implementation details (functions, variables, algorithms)
