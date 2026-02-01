# `yx sync` - Collaborate on Yaks via Git

Synchronizes yaks between team members using a hidden git ref (`refs/notes/yaks`). Enables distributed collaboration without polluting branch history.

## Usage

```bash
yx sync   # Push + pull + merge with origin

# Works in git worktrees
cd .worktrees/feature-branch
yx sync
```

Idempotent - safe to run anytime. Use before starting work, after changes, and periodically during collaboration.

## How It Works

**Storage: Hidden Git Ref**

Yaks stored in `refs/notes/yaks`:
- Lives in `.git/refs/notes/yaks` (never in branch history)
- Syncs to/from origin like branches
- Never appears in `git log`
- `.yaks/` stays untracked (in `.gitignore`)

**Sync Algorithm**:
1. Fetch `refs/notes/yaks` from origin
2. Commit any local changes
3. Merge local and remote refs (fast-forward when possible)
4. Push merged result to origin
5. Extract to `.yaks/` directory

**Merge Strategy**:
- Fast-forward if only one side changed
- True merge if both changed (uses git merge)
- Conflict resolution: **last-write-wins**

## Conflict Resolution: Last-Write-Wins

Concurrent edits to same yak: last sync wins.

```bash
# User A marks done, User B adds context (concurrent)
# After both sync: User B's version wins (last to sync)
```

Mitigate conflicts by syncing frequently and communicating before major changes.

## Git Worktrees

All worktrees share `refs/notes/yaks`:
```bash
cd .worktrees/feature-a
yx add "yak A" && yx sync

cd .worktrees/feature-b
yx sync  # Gets "yak A"
```

Perfect for multi-branch development.

## Key Behaviors

**No remote origin**: Sync succeeds silently (no-op), yaks stay local

**Prune + divergence**: Pruned yaks stay deleted (deletions preserved in merge)

**State changes**: Done/undo syncs correctly (state stored in files)

**No pollution**: Sync never touches staging area, working tree (except `.yaks/`), or branch history

## Troubleshooting

**Sync fails**: Check `git remote -v`, network access, credentials (SSH keys)

**Yaks not appearing**: Run `yx sync` again, check `git log refs/notes/yaks`

**Unexpected behavior**: File bug report with reproduction steps
