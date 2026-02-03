# Golden Master Test Fixtures

This directory contains expected output for visual format tests.

Files include ANSI escape codes for colors and formatting.

## Viewing Files

To see the actual colored output:
```bash
cat <filename> | less -R
```

## Updating Golden Masters

When intentionally changing output format:
1. Review the new output manually
2. Update the golden file
3. Commit with explanation of visual change
