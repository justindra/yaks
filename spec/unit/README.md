# Unit Tests

These are **implementation-specific tests** for internal functions and mechanics of the bash implementation.

## Purpose

Unit tests:
- Test bash functions directly
- Test internal implementation details
- Validate edge cases in function logic
- Are **specific to the bash implementation**

## When to Add Unit Tests

Add unit tests when:
- You need to test internal helper functions
- You want faster feedback on specific function behavior
- You're testing implementation details that aren't visible through the CLI

## When Rewriting in Another Language

These tests will need to be rewritten for each new implementation language:
- Zig implementation needs its own unit tests
- Go implementation needs its own unit tests
- etc.

The feature tests in `spec/features/` should remain valid across all implementations.

## Running Unit Tests

```bash
# Run only unit tests
shellspec spec/unit/

# Run a specific unit test
shellspec spec/unit/log_command.sh
```

## Writing Unit Tests

Unit tests can:
- Call bash functions directly
- Test internal state and variables
- Use mocking and stubbing
- Test implementation-specific edge cases
