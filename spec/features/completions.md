# `yx completions` - Shell Completion Support

Enables tab-completion of yak names in bash and zsh.

## Usage

```bash
# Install completions (adds source line to ~/.bashrc or ~/.zshrc)
yx completions install

# Dry run (preview changes)
yx completions install --dry-run

# Manual installation
# Add to ~/.bashrc or ~/.zshrc:
source /path/to/yaks/completions/yx.bash  # or yx.zsh
```

## Tab Completion

Once installed:
```bash
yx done <TAB>          # Shows incomplete yaks only
yx done --undo <TAB>   # Shows done yaks only
yx rm <TAB>            # Shows all yaks
yx context <TAB>       # Shows all yaks
```

## Behavior

- **Context-aware**: Filters by command (`done` shows incomplete, `done --undo` shows done)
- **Nested yaks**: Completes full paths (e.g., `parent/child`)
- **Auto-detection**: Detects shell from `$SHELL` (bash or zsh only)
- **Duplicate check**: Won't add source line twice
- **Alphabetical**: Suggestions sorted alphabetically

## Installation

Adds `source /path/to/yaks/completions/yx.bash` to your rc file. Auto-detects shell and rc file location.

**Errors**:
- Shell not bash/zsh: Fails with error, manual install required
- Already installed: Success message, no changes
- Cannot write rc file: Error with manual instructions

## Troubleshooting

**Not working after install**: Restart shell or `source ~/.bashrc`

**Wrong suggestions**: Clear cache (`hash -r` in bash, `rehash` in zsh)

**"Already installed" but broken**: Check `~/.bashrc` for stale path, remove and reinstall
