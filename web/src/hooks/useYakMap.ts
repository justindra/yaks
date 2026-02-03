import { signal, computed } from '@preact/signals';
import type { YakMap, Yak, Author } from '../lib/yak/types';
import {
  createEmptyYakMap,
  createYak,
  getDescendants,
  cloneYakMap,
  getChildren,
} from '../lib/yak/types';
import {
  validateMarkDone,
  validateDelete,
  validateMove,
  validateAdd,
} from '../lib/yak/validation';
import type { GitHubClient } from '../lib/git/client';
import { GitHubAPIError } from '../lib/git/client';
import { fetchYakMap } from '../lib/git/refs';
import { syncYakMap, pullYakMap } from '../lib/git/merge';

// Global yak map state
const yakMap = signal<YakMap | null>(null);
const baseYakMap = signal<YakMap | null>(null); // Last synced state for merge
const baseCommitOid = signal<string | null>(null);
const isLoading = signal(false);
const isSyncing = signal(false);
const error = signal<string | null>(null);
const syncError = signal<string | null>(null);

// Track if there are unsaved changes
const isDirty = computed(() => {
  if (!yakMap.value || !baseYakMap.value) {
    return yakMap.value !== null;
  }
  // Simple check: compare serialized versions
  return JSON.stringify(yakMapToJson(yakMap.value)) !== JSON.stringify(yakMapToJson(baseYakMap.value));
});

/**
 * Converts a YakMap to a plain object for comparison.
 */
function yakMapToJson(map: YakMap): object {
  const yaks: Record<string, Omit<Yak, 'id'>> = {};
  for (const [id, yak] of map.yaks) {
    yaks[id] = {
      name: yak.name,
      parentId: yak.parentId,
      done: yak.done,
      context: yak.context,
    };
  }
  return { yaks, roots: map.roots };
}

/**
 * Hook for yak map state and operations.
 */
export function useYakMap(client: GitHubClient | null) {
  /**
   * Loads the yak map from the repository.
   * Optionally accepts a client directly (useful when called right after connect).
   */
  const load = async (token?: string, clientOverride?: GitHubClient) => {
    const activeClient = clientOverride ?? client;
    if (!activeClient) {
      error.value = 'No repository connected';
      return;
    }

    isLoading.value = true;
    error.value = null;

    try {
      const result = await fetchYakMap(activeClient, token);
      
      if (result.yakMap) {
        yakMap.value = result.yakMap;
        baseYakMap.value = cloneYakMap(result.yakMap);
        baseCommitOid.value = result.commitOid;
      } else {
        // No yak map exists yet, start with empty
        yakMap.value = createEmptyYakMap();
        baseYakMap.value = createEmptyYakMap();
        baseCommitOid.value = null;
      }
    } catch (err) {
      if (err instanceof GitHubAPIError) {
        error.value = err.userMessage;
      } else {
        error.value = err instanceof Error ? err.message : 'Failed to load yak map';
      }
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Adds a new yak.
   */
  const addYak = (name: string, parentId?: string) => {
    if (!yakMap.value) return;

    const validationError = validateAdd(yakMap.value, name, parentId ?? null);
    if (validationError) {
      error.value = validationError;
      return;
    }

    const fullId = parentId ? `${parentId}/${name}` : name;
    const newYak = createYak(fullId, name, parentId ?? null);

    const newMap = cloneYakMap(yakMap.value);
    newMap.yaks.set(fullId, newYak);

    if (!parentId) {
      newMap.roots.push(fullId);
      newMap.roots.sort();
    }

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Updates a yak's properties.
   */
  const updateYak = (id: string, updates: Partial<Pick<Yak, 'context'>>) => {
    if (!yakMap.value) return;

    const yak = yakMap.value.yaks.get(id);
    if (!yak) {
      error.value = `Yak "${id}" not found`;
      return;
    }

    const newMap = cloneYakMap(yakMap.value);
    const updatedYak = { ...yak, ...updates };
    newMap.yaks.set(id, updatedYak);

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Deletes a yak.
   */
  const deleteYak = (id: string, recursive: boolean = false) => {
    if (!yakMap.value) return;

    if (!recursive) {
      const validationError = validateDelete(yakMap.value, id);
      if (validationError) {
        error.value = validationError;
        return;
      }
    }

    const newMap = cloneYakMap(yakMap.value);

    // Get all yaks to delete (including descendants if recursive)
    const toDelete = recursive
      ? [id, ...getDescendants(yakMap.value, id).map(y => y.id)]
      : [id];

    for (const yakId of toDelete) {
      newMap.yaks.delete(yakId);
    }

    // Update roots
    newMap.roots = newMap.roots.filter(r => !toDelete.includes(r));

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Moves a yak to a new parent.
   */
  const moveYak = (id: string, newParentId: string | null) => {
    if (!yakMap.value) return;

    const validationError = validateMove(yakMap.value, id, newParentId);
    if (validationError) {
      error.value = validationError;
      return;
    }

    const yak = yakMap.value.yaks.get(id)!;
    const oldParentId = yak.parentId;

    // Calculate new ID
    const newId = newParentId ? `${newParentId}/${yak.name}` : yak.name;

    if (newId === id) {
      // No change needed
      return;
    }

    const newMap = cloneYakMap(yakMap.value);

    // Function to update a yak and all its descendants
    const updateYakAndDescendants = (oldId: string, newIdPrefix: string) => {
      const y = newMap.yaks.get(oldId);
      if (!y) return;

      // Remove old entry
      newMap.yaks.delete(oldId);

      // Calculate new ID for this yak
      const suffix = oldId.substring(id.length); // Get the part after the original ID
      const updatedId = newIdPrefix + suffix;

      // Create updated yak
      const updatedYak: Yak = {
        ...y,
        id: updatedId,
        parentId: suffix === '' ? newParentId : updatedId.substring(0, updatedId.lastIndexOf('/')),
      };

      newMap.yaks.set(updatedId, updatedYak);

      // Update children
      const children = Array.from(newMap.yaks.values()).filter(
        c => c.parentId === oldId
      );
      for (const child of children) {
        updateYakAndDescendants(child.id, newIdPrefix);
      }
    };

    // Update the yak and all descendants
    updateYakAndDescendants(id, newId);

    // Update roots
    if (oldParentId === null && newParentId !== null) {
      // Was root, no longer root
      newMap.roots = newMap.roots.filter(r => r !== id);
    } else if (oldParentId !== null && newParentId === null) {
      // Wasn't root, now is root
      newMap.roots.push(newId);
      newMap.roots.sort();
    } else if (oldParentId === null && newParentId === null) {
      // Still root, but ID changed
      newMap.roots = newMap.roots.map(r => r === id ? newId : r).sort();
    }

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Marks a yak as done or not done.
   */
  const markDone = (id: string, done: boolean, recursive: boolean = false) => {
    if (!yakMap.value) return;

    if (done) {
      const validationError = validateMarkDone(yakMap.value, id, recursive);
      if (validationError) {
        error.value = validationError;
        return;
      }
    }

    const newMap = cloneYakMap(yakMap.value);

    if (recursive && done) {
      // Mark this yak and all descendants as done
      const toMark = [id, ...getDescendants(yakMap.value, id).map(y => y.id)];
      for (const yakId of toMark) {
        const y = newMap.yaks.get(yakId);
        if (y) {
          newMap.yaks.set(yakId, { ...y, done: true });
        }
      }
    } else {
      const yak = newMap.yaks.get(id);
      if (yak) {
        newMap.yaks.set(id, { ...yak, done });
      }
    }

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Removes all done yaks.
   */
  const prune = () => {
    if (!yakMap.value) return;

    const newMap = cloneYakMap(yakMap.value);

    // Find all done yaks that have no incomplete descendants
    const canPrune = (id: string): boolean => {
      const yak = newMap.yaks.get(id);
      if (!yak || !yak.done) return false;

      const children = getChildren(newMap, id);
      return children.every(child => canPrune(child.id));
    };

    // Collect yaks to prune (done and all descendants are done)
    const toPrune: string[] = [];
    for (const [id, yak] of newMap.yaks) {
      if (yak.done && canPrune(id)) {
        toPrune.push(id);
      }
    }

    // Remove them
    for (const id of toPrune) {
      newMap.yaks.delete(id);
    }

    // Update roots
    newMap.roots = newMap.roots.filter(r => newMap.yaks.has(r));

    yakMap.value = newMap;
    error.value = null;
  };

  /**
   * Pulls latest from remote.
   */
  const pull = async (token?: string) => {
    if (!client || !yakMap.value) return;

    isSyncing.value = true;
    syncError.value = null;

    try {
      const result = await pullYakMap(client, yakMap.value, baseYakMap.value, token);

      if (result.yakMap) {
        yakMap.value = result.yakMap;
        baseYakMap.value = cloneYakMap(result.yakMap);
        baseCommitOid.value = result.commitOid;
      }

      if (result.hadConflicts) {
        syncError.value = 'Conflicts were auto-resolved (local changes kept)';
      }
    } catch (err) {
      if (err instanceof GitHubAPIError) {
        syncError.value = err.userMessage;
      } else {
        syncError.value = err instanceof Error ? err.message : 'Failed to pull';
      }
    } finally {
      isSyncing.value = false;
    }
  };

  /**
   * Pushes local changes to remote.
   */
  const push = async (token: string, authorName?: string, authorEmail?: string) => {
    if (!client || !yakMap.value) return;

    isSyncing.value = true;
    syncError.value = null;

    const author: Author = {
      name: authorName || 'Yak Map User',
      email: authorEmail || 'yak-map@example.com',
    };

    try {
      const result = await syncYakMap(
        client,
        yakMap.value,
        baseYakMap.value,
        baseCommitOid.value,
        author,
        token
      );

      // Update base state
      baseYakMap.value = cloneYakMap(yakMap.value);
      baseCommitOid.value = result.commitSha ?? baseCommitOid.value;

      if (result.hadConflicts) {
        syncError.value = 'Conflicts were auto-resolved (local changes kept)';
      }
    } catch (err) {
      if (err instanceof GitHubAPIError) {
        syncError.value = err.userMessage;
      } else {
        syncError.value = err instanceof Error ? err.message : 'Failed to push';
      }
    } finally {
      isSyncing.value = false;
    }
  };

  return {
    // State
    yakMap,
    baseYakMap,
    isLoading,
    isSyncing,
    error,
    syncError,
    isDirty,
    
    // Actions
    load,
    addYak,
    updateYak,
    deleteYak,
    moveYak,
    markDone,
    prune,
    pull,
    push,
  };
}
