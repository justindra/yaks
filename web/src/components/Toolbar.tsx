export type ViewMode = 'graph' | 'tree';

interface ToolbarProps {
  // Yak actions
  onAddYak: () => void;
  onPrune: () => void;
  hasDoneYaks: boolean;
  // Sync state
  isDirty: boolean;
  isSyncing: boolean;
  lastSyncError: string | null;
  isAuthenticated: boolean;
  onPull: () => void;
  onPush: () => void;
  // View mode
  viewMode: ViewMode;
  onViewModeChange: (mode: ViewMode) => void;
}

export function Toolbar({
  onAddYak,
  onPrune,
  hasDoneYaks,
  isDirty,
  isSyncing,
  lastSyncError,
  isAuthenticated,
  onPull,
  onPush,
  viewMode,
  onViewModeChange,
}: ToolbarProps) {
  return (
    <div class="flex items-center justify-between p-sm px-md border-b bg-secondary">
      {/* Left side: Sync controls */}
      <div class="flex items-center gap-sm">
        <button
          onClick={onPull}
          disabled={isSyncing}
          class="small"
          title="Pull latest changes from remote"
        >
          Pull
        </button>
        <button
          onClick={onPush}
          disabled={isSyncing || !isDirty || !isAuthenticated}
          class="small primary"
          title={
            !isAuthenticated
              ? 'Login required to push'
              : !isDirty
                ? 'No changes to push'
                : 'Push changes to remote'
          }
        >
          Push
        </button>

        {/* Status indicator */}
        <div class="flex items-center gap-sm ml-sm">
          {isSyncing ? (
            <>
              <span class="spinner" />
              <span class="text-sm">Syncing...</span>
            </>
          ) : isDirty ? (
            <>
              <span style={{
                width: '8px',
                height: '8px',
                borderRadius: '50%',
                background: 'var(--color-warning)'
              }} />
              <span class="text-sm text-warning">Unsaved changes</span>
            </>
          ) : (
            <>
              <span style={{
                width: '8px',
                height: '8px',
                borderRadius: '50%',
                background: 'var(--color-success)'
              }} />
              <span class="text-sm text-muted">Synced</span>
            </>
          )}

          {/* Error display */}
          {lastSyncError && (
            <span class="text-sm text-error">
              {lastSyncError}
            </span>
          )}
        </div>
      </div>

      {/* Center: View mode toggle */}
      <div class="flex items-center gap-xs">
        <button
          onClick={() => onViewModeChange('tree')}
          class={`small ${viewMode === 'tree' ? 'primary' : ''}`}
          title="Tree view"
        >
          Tree
        </button>
        <button
          onClick={() => onViewModeChange('graph')}
          class={`small ${viewMode === 'graph' ? 'primary' : ''}`}
          title="Graph view"
        >
          Graph
        </button>
      </div>

      {/* Right side: Yak actions */}
      <div class="flex items-center gap-sm">
        <button onClick={onAddYak} class="small primary">
          + Add Yak
        </button>
        <button
          onClick={onPrune}
          disabled={!hasDoneYaks}
          class="small"
          title={hasDoneYaks ? 'Remove all completed yaks' : 'No completed yaks to prune'}
        >
          Prune Done
        </button>
      </div>
    </div>
  );
}
