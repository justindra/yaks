# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# Yak - DAG-based TODO List CLI

A CLI tool for managing TODO lists as a directed acyclic graph (DAG), designed for teams working on software projects. The name comes from "yak shaving" - when you set out to do task A but discover you need B first, which requires C.

## Core Commands

```bash
# Testing
shellspec                    # Run all tests
shellspec spec/list.sh       # Run specific test file

# Development
yx add <name>                # Add a yak
yx ls                        # List yaks
yx context <name>            # Edit context (uses $EDITOR or stdin)
yx done <name>               # Mark complete
yx rm <name>                 # Remove a yak
yx prune                     # Remove all done yaks
```

The command is `yx` (installed in PATH via direnv), not `./yx`.

## Architecture

### Single-File CLI
All logic is in `bin/yx` - a ~240 line bash script organized into functions:
- Command routing via case statement (line 199+)
- Each command is a function (list_yaks, add_yak, done_yak, etc.)
- Storage: `.yaks/<yak-name>/` directories with optional `done` marker and `context.md`

### Storage Pattern
- Uses `YAK_PATH` environment variable (defaults to `.yaks`)
- Each yak is a directory: `$YAK_DIR/<yak-name>/`
- `done` file marks completion
- `context.md` holds additional notes
- Adapter pattern allows future backends (git refs planned)

### Testing
- Framework: ShellSpec
- Pattern: Each command has its own spec file (spec/add.sh, spec/list.sh, etc.)
- Tests use `YAK_PATH=$(mktemp -d)` for isolation
- Configuration: `.shellspec` sets format, pattern, and shell

## Development Workflow

**Test-Driven Development (TDD)**:
1. Write ONE failing test
2. Run `shellspec` (RED)
3. Implement minimal code to pass (GREEN)
4. Run `shellspec` to verify
5. Refactor if needed
6. Commit
7. Repeat

**TRUST THE TESTS**: When tests pass, the feature works. Do NOT run redundant manual verification.

**Incremental approach**: Use the `incremental-tdd` skill for guidance on writing one test at a time.

## CRITICAL: Dogfooding Rule

**NEVER touch the `.yaks` folder in this project!**

We're using yaks to build yaks (dogfooding). The `.yaks` folder contains the actual work tracker for this project.

- **For testing**: Use `YAK_PATH` (tests set this to temp directories)
- **For demos**: Use `YAK_PATH=/tmp/demo-yaks yx <command>`
- **NEVER**: Run `rm -rf .yaks` or modify `.yaks` contents directly

## Multi-Agent Workflow

When working on yaks in a multi-agent environment, use the **`yak-worktree-workflow` skill** for:
- Creating isolated git worktrees for each yak
- Reading yak context before starting
- Asking for clarification if context is insufficient
- Merging work back to main when complete

## Commit Message Policy

**Do NOT include Claude's name or "Co-Authored-By: Claude" in commit messages.**

Commits should be clean and professional without AI attribution.

## Future Vision

The current implementation is Phase 1 (directory-based storage). Future plans include:
- Git ref backend for cross-branch collaboration
- Hierarchy/containment model (yaks contain sub-yaks)
- Team swarming capability (visibility into who's working on what)

Currently out of scope: time tracking, priority levels, rich text, external integrations, auth, cloud sync.
