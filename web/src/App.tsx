import { signal } from '@preact/signals';
import { AuthStatus } from './components/AuthStatus';
import { PATModal } from './components/PATModal';
import { RepoInput } from './components/RepoInput';
import { YakGraph } from './components/YakGraph';
import { YakTree } from './components/YakTree';
import { YakEditor } from './components/YakEditor';
import { Toolbar, ViewMode } from './components/Toolbar';
import { AddYakModal } from './components/AddYakModal';
import { useAuth } from './hooks/useAuth';
import { useRepo } from './hooks/useRepo';
import { useYakMap } from './hooks/useYakMap';

// Global UI state
const selectedYakId = signal<string | null>(null);
const showAddModal = signal(false);
const viewMode = signal<ViewMode>('tree'); // Default to tree view

export function App() {
  const auth = useAuth();
  const repo = useRepo();
  const yakMap = useYakMap(repo.client.value);

  const handleRepoSubmit = async (url: string) => {
    const client = await repo.connect(url, auth.token.value ?? undefined);
    if (client) {
      await yakMap.load(undefined, client);
    }
  };

  const handleSelectYak = (id: string | null) => {
    selectedYakId.value = id;
  };

  const handleMoveYak = (id: string, newParentId: string | null) => {
    auth.requireAuth(() => {
      yakMap.moveYak(id, newParentId);
    });
  };

  const handleAddYak = (name: string, parentId?: string) => {
    // Auth already required when opening the modal
    yakMap.addYak(name, parentId);
    showAddModal.value = false;
  };

  const handleOpenAddModal = () => {
    auth.requireAuth(() => {
      showAddModal.value = true;
    });
  };

  const handleToggleDone = (id: string) => {
    auth.requireAuth(() => {
      const yak = yakMap.yakMap.value?.yaks.get(id);
      if (yak) yakMap.markDone(id, !yak.done);
    });
  };

  const selectedYak = selectedYakId.value && yakMap.yakMap.value
    ? yakMap.yakMap.value.yaks.get(selectedYakId.value) ?? null
    : null;

  // Shared props for both graph and tree views
  const viewProps = {
    yakMap: yakMap.yakMap.value!,
    selectedId: selectedYakId.value,
    onSelect: handleSelectYak,
    onMove: handleMoveYak,
    onToggleDone: handleToggleDone,
  };

  return (
    <div class="flex flex-col" style={{ height: '100%' }}>
      {/* Header */}
      <header class="flex items-center justify-between p-md bg-secondary border-b flex-shrink-0">
        <div class="flex items-center gap-md">
          <h1 class="font-bold" style={{ margin: 0 }}>Yak Map</h1>
          <RepoInput
            onSubmit={handleRepoSubmit}
            isLoading={repo.isLoading.value}
            isConnected={repo.isConnected.value}
          />
        </div>
        <AuthStatus
          isAuthenticated={auth.isAuthenticated.value}
          username={auth.username.value}
          onLogout={auth.logout}
        />
      </header>

      {/* Toolbar with sync controls */}
      {repo.isConnected.value && (
        <Toolbar
          onAddYak={handleOpenAddModal}
          onPrune={() => auth.requireAuth(() => yakMap.prune())}
          hasDoneYaks={yakMap.yakMap.value ? 
            Array.from(yakMap.yakMap.value.yaks.values()).some(y => y.done) : false}
          isDirty={yakMap.isDirty.value}
          isSyncing={yakMap.isSyncing.value}
          lastSyncError={yakMap.syncError.value}
          isAuthenticated={auth.isAuthenticated.value}
          onPull={() => yakMap.pull(auth.token.value ?? undefined)}
          onPush={() => auth.requireAuth(() => yakMap.push(auth.token.value!))}
          viewMode={viewMode.value}
          onViewModeChange={(mode) => viewMode.value = mode}
        />
      )}

      {/* Main content */}
      <div class="flex flex-1" style={{ overflow: 'hidden' }}>
        {/* Graph/Tree area */}
        <main class="flex-1 flex flex-col" style={{ overflow: 'hidden' }}>
          <div class="flex-1" style={{ position: 'relative', overflow: 'hidden' }}>
            {repo.isConnected.value && yakMap.yakMap.value ? (
              viewMode.value === 'tree' ? (
                <YakTree {...viewProps} />
              ) : (
                <YakGraph {...viewProps} />
              )
            ) : (
              <div class="flex items-center justify-center h-full text-muted">
                {repo.isLoading.value ? (
                  <div class="flex items-center gap-md">
                    <div class="spinner" />
                    <span>Loading repository...</span>
                  </div>
                ) : yakMap.isLoading.value ? (
                  <div class="flex items-center gap-md">
                    <div class="spinner" />
                    <span>Loading yak map...</span>
                  </div>
                ) : (
                  <span>Enter a GitHub repository URL to view its yak map</span>
                )}
              </div>
            )}

            {/* Error display */}
            {(repo.error.value || yakMap.error.value) && (
              <div class="p-md bg-tertiary rounded text-error" style={{ 
                position: 'absolute', 
                bottom: 'var(--spacing-md)', 
                left: 'var(--spacing-md)', 
                right: 'var(--spacing-md)' 
              }}>
                {repo.error.value || yakMap.error.value}
              </div>
            )}
          </div>
        </main>

        {/* Sidebar - Yak Editor */}
        {selectedYak && yakMap.yakMap.value && (
          <aside class="bg-secondary border-l flex-shrink-0" style={{ width: '320px', overflow: 'auto' }}>
            <YakEditor
              yak={selectedYak}
              yakMap={yakMap.yakMap.value}
              onUpdate={(updates) => {
                auth.requireAuth(() => yakMap.updateYak(selectedYak.id, updates));
              }}
              onToggleDone={(done) => {
                auth.requireAuth(() => yakMap.markDone(selectedYak.id, done));
              }}
              onAddChild={() => {
                auth.requireAuth(() => {
                  showAddModal.value = true;
                  // selectedYakId is already set, AddYakModal uses it as selectedParentId
                });
              }}
              onDelete={() => {
                auth.requireAuth(() => {
                  yakMap.deleteYak(selectedYak.id);
                  selectedYakId.value = null;
                });
              }}
              onClose={() => selectedYakId.value = null}
            />
          </aside>
        )}
      </div>

      {/* Footer - Help text */}
      {repo.isConnected.value && (
        <footer class="bg-secondary border-t flex-shrink-0 p-sm px-md">
          <span class="text-xs text-muted">
            Click to select · Double-click to toggle done · Drag to reparent
          </span>
        </footer>
      )}

      {/* Add Yak Modal */}
      {showAddModal.value && yakMap.yakMap.value && (
        <AddYakModal
          yakMap={yakMap.yakMap.value}
          selectedParentId={selectedYakId.value}
          onAdd={handleAddYak}
          onClose={() => showAddModal.value = false}
        />
      )}

      {/* PAT Modal */}
      <PATModal
        isOpen={auth.showPATModal.value}
        isLoading={auth.isLoading.value}
        error={auth.error.value}
        onSave={auth.saveToken}
        onClose={auth.closePATModal}
      />
    </div>
  );
}
